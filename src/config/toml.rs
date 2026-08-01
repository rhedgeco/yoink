use std::{
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use toml::{Table, Value};

use crate::Yoink;

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlConfig {
    path: PathBuf,
    #[serde(default)]
    overrides: Vec<PathBuf>,
}

impl Yoink for TomlConfig {
    fn pull(&mut self, _: Option<&[u8]>) -> anyhow::Result<Box<[u8]>> {
        let path = &self.path;

        // extract the toml table from the main file
        let content = fs::read_to_string(path).map_err(path_err(path))?;
        let mut table: Table = toml::from_str(&content).map_err(path_err(path))?;

        // collect all override tables
        let mut overrides = Vec::with_capacity(self.overrides.len());
        for path in self.overrides.iter().map(PathBuf::as_path) {
            let content = match fs::read_to_string(path) {
                Ok(content) => content,
                // if the file is not found, we can just skip it
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => return Err(path_err(path)(err)),
            };

            let table: Table = toml::from_str(&content).map_err(path_err(path))?;
            overrides.push(table);
        }

        // merge all overrides into the first table
        for table_override in overrides {
            merge_tables(&mut table, table_override);
        }

        // render the compiled table into an output
        let output = toml::to_string_pretty(&table)?;
        Ok(output.into_bytes().into_boxed_slice())
    }
}

fn path_err<E: Display>(path: impl AsRef<Path>) -> impl FnOnce(E) -> anyhow::Error {
    move |err| anyhow!("'{}': {err}", path.as_ref().display())
}

fn merge_values(value1: &mut Value, value2: Value) {
    match (value1, value2) {
        // merge tables if both values are tables
        (Value::Table(table1), Value::Table(table2)) => {
            merge_tables(table1, table2);
        }
        // if they are anything else, override value1
        (value1, value2) => *value1 = value2,
    }
}

fn merge_tables(table1: &mut Table, table2: Table) {
    for (key, value) in table2 {
        use toml::map::Entry as E;
        match table1.entry(key) {
            // if its vacant, we can just insert it
            E::Vacant(entry) => {
                entry.insert(value);
            }
            // if its occupied, try merging the values
            E::Occupied(entry) => {
                merge_values(entry.into_mut(), value);
            }
        };
    }
}
