use std::fs::File;
use std::fs::TryLockError;
use std::path::Path;

use anyhow::Result;

const LOCK_EXTENSION: &str = "lock";

pub struct InstanceLock {
    _file: File
}

impl InstanceLock {
    pub fn acquire(output: &Path) -> Result<Option<Self>> {
        let mut path = output.as_os_str().to_owned();
        path.push(format!(".{LOCK_EXTENSION}"));
        let file = File::create(path)?;

        match file.try_lock() {
            Ok(()) => Ok(Some(Self { _file: file })),
            Err(TryLockError::WouldBlock) => Ok(None),
            Err(TryLockError::Error(error)) => Err(error.into())
        }
    }
}
