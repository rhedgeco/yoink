use std::{fs, io, path::PathBuf};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use super::Runner;

#[derive(Debug, Serialize, Deserialize)]
pub struct BytesConfig {
    path: PathBuf,
}

impl Runner for BytesConfig {
    fn yoink(&self, mut target: impl io::Write) -> anyhow::Result<()> {
        let path = &self.path;

        // read the bytes directly from the target path
        let bytes = fs::read(path).map_err(|err| {
            let path_display = path.display();
            anyhow!("'{path_display}': {err}")
        })?;

        // then write the bytes to the target directly
        target.write(&bytes)?;
        Ok(())
    }
}
