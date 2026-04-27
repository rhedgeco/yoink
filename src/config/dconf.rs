use std::{
    fmt::Display,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use gvdb::read::File;
use serde::{Deserialize, Serialize};

use crate::Yoink;

#[derive(Debug, Serialize, Deserialize)]
pub struct DconfConfig {
    path: PathBuf,
    #[serde(default)]
    include: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

impl Yoink for DconfConfig {
    fn pull(&mut self, _: Option<&[u8]>) -> anyhow::Result<Box<[u8]>> {
        let path = &self.path;

        let file = File::from_file(path)?;
        let table = file.hash_table().map_err(path_err(path))?;

        let mut bytes = Vec::new();
        for key in table.keys() {
            // convert the key to a string
            let key = key.map_err(path_err(path))?.to_string();

            // exclude any keys that dont conatin an included prefix
            if !self.include.iter().any(|str| key.starts_with(str)) {
                continue;
            }

            // exclude any keys that contain an excluded prefix
            if self.exclude.iter().any(|str| key.starts_with(str)) {
                continue;
            }

            // get the value for the key
            // skip if there is an error for this key
            let Ok(value) = table.get_value(&key) else {
                continue;
            };

            // write the key value line to the target
            writeln!(bytes, "{key} = {value}")?;
        }

        Ok(bytes.into_boxed_slice())
    }
}

fn path_err<E: Display>(path: impl AsRef<Path>) -> impl FnOnce(E) -> anyhow::Error {
    move |err| anyhow!("'{}': {err}", path.as_ref().display())
}
