use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use tempfile::NamedTempFile;

pub struct FrameWriter {
    path: PathBuf,
    directory: PathBuf,
    last: Option<Vec<u8>>
}

impl FrameWriter {
    pub fn new(path: PathBuf) -> Result<Self> {
        let directory = path
            .parent()
            .context("output has no parent directory")?
            .to_path_buf();
        fs::create_dir_all(&directory)?;

        Ok(Self {
            path,
            directory,
            last: None
        })
    }

    pub fn write(&mut self, frame: Vec<u8>) -> Result<()> {
        if self.last.as_ref() == Some(&frame) {
            return Ok(());
        }

        let mut file = NamedTempFile::new_in(&self.directory)?;
        file.write_all(&frame)?;
        file.persist(&self.path)?;
        self.last = Some(frame);

        Ok(())
    }
}
