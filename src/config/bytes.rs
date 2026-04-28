use std::{fs, path::PathBuf};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::Yoink;

#[derive(Debug, Serialize, Deserialize)]
pub struct BytesConfig {
    path: PathBuf,
}

impl Yoink for BytesConfig {
    fn pull(&mut self, _: Option<&[u8]>) -> anyhow::Result<Box<[u8]>> {
        let path = &self.path;

        // read the bytes directly from the target path
        let bytes = fs::read(path).map_err(|err| {
            let path_display = path.display();
            anyhow!("'{path_display}': {err}")
        })?;

        Ok(bytes.into_boxed_slice())
    }
}
