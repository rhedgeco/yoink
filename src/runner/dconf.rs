use std::{
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use gvdb::read::File;
use serde::{Deserialize, Serialize};

use super::Runner;

#[derive(Debug, Serialize, Deserialize)]
pub struct DconfConfig {
    path: PathBuf,
    #[serde(default)]
    exclude: Vec<String>,
}

impl Runner for DconfConfig {
    fn yoink(&self, mut target: impl io::Write) -> anyhow::Result<()> {
        let path = &self.path;

        let file = File::from_file(path).map_err(path_err(path))?;
        let table = file.hash_table().map_err(path_err(path))?;

        for key in table.keys() {
            // convert the key to a string
            let key = key.map_err(path_err(path))?.to_string();

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
            writeln!(target, "{key} = {value}")?;
        }

        Ok(())
    }
}

fn path_err<E: Display>(path: impl AsRef<Path>) -> impl FnOnce(E) -> anyhow::Error {
    move |err| anyhow!("'{}': {err}", path.as_ref().display())
}
