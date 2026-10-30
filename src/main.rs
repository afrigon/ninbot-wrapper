mod arguments;
mod game;
mod instance_lock;
mod ninjabrain;
mod overlay;
mod service;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::arguments::Arguments;
use crate::service::Service;

const DEFAULT_LOG_FILTER: &str = "info";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_FILTER))
        )
        .with_writer(std::io::stderr)
        .init();

    Service::new(Arguments::parse()).run().await
}
