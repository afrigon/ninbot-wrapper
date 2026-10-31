use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use directories::ProjectDirs;
use reqwest::Client;
use tempfile::NamedTempFile;
use tracing::info;

// Speedrun rules whitelist specific Ninjabrain Bot versions, so the version only changes deliberately.
const VERSION: &str = "1.5.2";
const DOWNLOAD_URL: &str = "https://github.com/Ninjabrain1/Ninjabrain-Bot/releases/download";

pub struct Release {
    client: Client,
    cache: PathBuf
}

impl Release {
    pub fn new(client: Client) -> Result<Self> {
        let directories = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .context("no home directory to cache Ninjabrain Bot in")?;

        Ok(Self {
            client,
            cache: directories.cache_dir().to_path_buf()
        })
    }

    pub async fn jar(&self) -> Result<PathBuf> {
        let name = format!("Ninjabrain-Bot-{VERSION}.jar");
        let jar = self.cache.join(&name);

        if jar.exists() {
            return Ok(jar);
        }

        info!("downloading {name}");

        let bytes = self
            .client
            .get(format!("{DOWNLOAD_URL}/{VERSION}/{name}"))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        fs::create_dir_all(&self.cache)?;
        let mut file = NamedTempFile::new_in(&self.cache)?;
        file.write_all(&bytes)?;
        file.persist(&jar)?;

        Ok(jar)
    }
}
