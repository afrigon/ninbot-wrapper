use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Arguments {
    #[arg(help = "PNG the overlay is rendered to")]
    pub output: PathBuf,

    #[arg(help = "Game state file written by the State Output mod (wpstateout.txt)")]
    pub state: PathBuf
}
