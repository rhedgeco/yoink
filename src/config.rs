use serde::{Deserialize, Serialize};

use crate::runner::{Runner, bytes::BytesConfig, dconf::DconfConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub runner: RunnerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunnerConfig {
    Bytes(BytesConfig),
    Dconf(DconfConfig),
}

impl Runner for RunnerConfig {
    fn yoink(&self, target: impl std::io::Write) -> anyhow::Result<()> {
        match self {
            RunnerConfig::Bytes(config) => config.yoink(target),
            RunnerConfig::Dconf(config) => config.yoink(target),
        }
    }
}
