use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use notify::Event;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use tokio::sync::watch;
use tracing::warn;

use super::GameState;

pub struct StateWatcher {
    _watcher: RecommendedWatcher
}

impl StateWatcher {
    pub fn start(path: PathBuf, sender: watch::Sender<Option<GameState>>) -> Result<Self> {
        let directory = path
            .parent()
            .context("state file has no parent directory")?
            .to_path_buf();
        let name = path
            .file_name()
            .context("state file has no file name")?
            .to_owned();

        Self::publish(&path, &sender);

        let mut watcher =
            notify::recommended_watcher(move |event: notify::Result<Event>| match event {
                Ok(event)
                    if event
                        .paths
                        .iter()
                        .any(|changed| changed.file_name() == Some(&name)) =>
                {
                    Self::publish(&path, &sender);
                }
                Ok(_) => {}
                Err(error) => warn!("watching the game state failed: {error}")
            })?;
        watcher.watch(&directory, RecursiveMode::NonRecursive)?;

        Ok(Self { _watcher: watcher })
    }

    fn publish(path: &Path, sender: &watch::Sender<Option<GameState>>) {
        let state = Self::read(path);

        sender.send_if_modified(|current| {
            let changed = *current != state;
            *current = state;
            changed
        });
    }

    fn read(path: &Path) -> Option<GameState> {
        match fs::read_to_string(path) {
            Ok(contents) => contents
                .parse()
                .inspect_err(|error| warn!("{error:#}"))
                .ok(),
            Err(error) if error.kind() == ErrorKind::NotFound => None,
            Err(error) => {
                warn!("could not read {}: {error}", path.display());
                None
            }
        }
    }
}
