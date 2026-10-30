use std::path::Path;
use std::process::ExitStatus;
use std::process::Stdio;

use anyhow::Result;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncRead;
use tokio::io::BufReader;
use tokio::process::Child;
use tokio::process::Command;
use tracing::info;

use super::ChildGuard;

const JAVA: &str = "java";

pub struct BotProcess {
    child: Child,
    _guard: ChildGuard
}

impl BotProcess {
    pub fn spawn(jar: &Path) -> Result<Self> {
        let mut command = Command::new(JAVA);
        command
            .arg("-jar")
            .arg(jar)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        ChildGuard::prepare(&mut command);

        let mut child = command.spawn()?;
        let guard = ChildGuard::attach(&child)?;

        if let Some(stdout) = child.stdout.take() {
            tokio::spawn(Self::forward(stdout));
        }

        if let Some(stderr) = child.stderr.take() {
            tokio::spawn(Self::forward(stderr));
        }

        info!("started Ninjabrain Bot from {}", jar.display());

        Ok(Self {
            child,
            _guard: guard
        })
    }

    pub async fn wait(&mut self) -> Result<ExitStatus> {
        Ok(self.child.wait().await?)
    }

    async fn forward(output: impl AsyncRead + Unpin) {
        let mut lines = BufReader::new(output).lines();

        while let Ok(Some(line)) = lines.next_line().await {
            info!(target: "ninjabrain", "{line}");
        }
    }
}
