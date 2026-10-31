use std::fs;
use std::io::ErrorKind;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use tempfile::NamedTempFile;

use super::Frame;

pub struct FrameWriter {
    path: PathBuf,
    directory: PathBuf,
    last: Option<Frame>
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

    pub fn write(&mut self, frame: Frame) -> Result<()> {
        if self.last.as_ref() == Some(&frame) {
            return Ok(());
        }

        match &frame {
            Frame::Empty => self.remove()?,
            Frame::Image(image) => self.replace(image)?
        }

        self.last = Some(frame);

        Ok(())
    }

    fn replace(&self, image: &[u8]) -> Result<()> {
        let mut file = NamedTempFile::new_in(&self.directory)?;
        file.write_all(image)?;
        file.persist(&self.path)?;

        Ok(())
    }

    fn remove(&self) -> Result<()> {
        match fs::remove_file(&self.path) {
            Err(error) if error.kind() != ErrorKind::NotFound => Err(error.into()),
            _ => Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_frame_removes_the_image() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("overlay.png");
        let mut writer = FrameWriter::new(path.clone()).unwrap();

        writer.write(Frame::Image(vec![1, 2, 3])).unwrap();
        assert_eq!(fs::read(&path).unwrap(), [1, 2, 3]);

        writer.write(Frame::Empty).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn an_empty_frame_without_an_image_succeeds() {
        let directory = tempfile::tempdir().unwrap();
        let mut writer = FrameWriter::new(directory.path().join("overlay.png")).unwrap();

        writer.write(Frame::Empty).unwrap();
    }
}
