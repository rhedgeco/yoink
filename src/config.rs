use serde::{Deserialize, Serialize};

use crate::Yoink;

mod bytes;
mod dconf;
mod toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    Bytes(bytes::BytesConfig),
    Dconf(dconf::DconfConfig),
    Toml(toml::TomlConfig),
}

impl Yoink for Target {
    fn pull(&mut self, store: Option<&[u8]>) -> anyhow::Result<Box<[u8]>> {
        match self {
            Target::Bytes(config) => config.pull(store),
            Target::Dconf(config) => config.pull(store),
            Target::Toml(config) => config.pull(store),
        }
    }
}
