use std::time::Duration;

use anyhow::Result;
use futures_util::StreamExt;
use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio::sync::watch;
use tokio::time;
use tracing::debug;
use tracing::warn;

const API_URL: &str = "http://localhost:52533/api/v1";
const EVENTS_SUFFIX: &str = "/events";
const DATA_FIELD: &str = "data:";
const RECONNECT_DELAY: Duration = Duration::from_secs(1);

pub struct EventStream {
    client: Client,
    endpoint: &'static str
}

impl EventStream {
    pub fn new(client: Client, endpoint: &'static str) -> Self {
        Self { client, endpoint }
    }

    pub async fn run<T: DeserializeOwned>(self, sender: watch::Sender<T>) {
        loop {
            if let Err(error) = self.follow(&sender).await {
                debug!("{} stream disconnected: {error:#}", self.endpoint);
            }

            time::sleep(RECONNECT_DELAY).await;
        }
    }

    async fn follow<T: DeserializeOwned>(&self, sender: &watch::Sender<T>) -> Result<()> {
        let url = format!("{API_URL}{}", self.endpoint);
        let snapshot = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        sender.send_replace(snapshot);

        let mut body = self
            .client
            .get(format!("{url}{EVENTS_SUFFIX}"))
            .send()
            .await?
            .error_for_status()?
            .bytes_stream();
        let mut buffer = Vec::new();
        let mut data = String::new();

        while let Some(chunk) = body.next().await {
            buffer.extend_from_slice(&chunk?);

            while let Some(end) = buffer.iter().position(|byte| *byte == b'\n') {
                let line: Vec<u8> = buffer.drain(..=end).collect();
                let line = std::str::from_utf8(&line)?.trim_end_matches(['\r', '\n']);
                self.read_line(line, &mut data, sender);
            }
        }

        Ok(())
    }

    fn read_line<T: DeserializeOwned>(
        &self,
        line: &str,
        data: &mut String,
        sender: &watch::Sender<T>
    ) {
        if line.is_empty() {
            if !data.is_empty() {
                match serde_json::from_str(data) {
                    Ok(event) => {
                        sender.send_replace(event);
                    }
                    Err(error) => warn!("{} sent an unreadable event: {error}", self.endpoint)
                }

                data.clear();
            }

            return;
        }

        if let Some(value) = line.strip_prefix(DATA_FIELD) {
            if !data.is_empty() {
                data.push('\n');
            }

            data.push_str(value.strip_prefix(' ').unwrap_or(value));
        }
    }
}
