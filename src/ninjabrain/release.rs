use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use directories::ProjectDirs;
use reqwest::Client;
use serde::Deserialize;
use tempfile::NamedTempFile;
use tracing::info;
use tracing::warn;

const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/Ninjabrain1/Ninjabrain-Bot/releases/latest";
const JAR_EXTENSION: &str = "jar";

pub struct Release {
    client: Client,
    cache: PathBuf
}

#[derive(Deserialize)]
struct GithubRelease {
    assets: Vec<GithubAsset>
}

#[derive(Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String
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
        match self.latest().await {
            Ok(jar) => Ok(jar),
            Err(error) => {
                warn!("could not fetch the latest Ninjabrain Bot release: {error:#}");
                self.newest_cached()
                    .context("no cached Ninjabrain Bot release to fall back on")
            }
        }
    }

    async fn latest(&self) -> Result<PathBuf> {
        let release: GithubRelease = self
            .client
            .get(LATEST_RELEASE_URL)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let asset = release
            .assets
            .into_iter()
            .find(|asset| asset.name.ends_with(&format!(".{JAR_EXTENSION}")))
            .context("latest release has no jar asset")?;

        let jar = self.cache.join(&asset.name);

        if jar.exists() {
            return Ok(jar);
        }

        info!("downloading {}", asset.name);

        let bytes = self
            .client
            .get(&asset.browser_download_url)
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

    fn newest_cached(&self) -> Option<PathBuf> {
        fs::read_dir(&self.cache)
            .ok()?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == JAR_EXTENSION)
            })
            .max_by_key(|path| {
                path.metadata()
                    .and_then(|metadata| metadata.modified())
                    .ok()
            })
    }
}
