use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub target: Target,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub style: Style,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    Bytes { path: PathBuf },
}
