use std::{env, path::PathBuf};

use anyhow::anyhow;
use clap::Parser;
use yoink::yoink;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// Recurse subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Target file or directory to be yoinked (Defaults to current dir)
    pub target: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    // parse the command line args
    let cli = Cli::parse();

    // use the target path, or the current working directory
    let path = match cli.target {
        Some(path) => path,
        None => env::current_dir().map_err(|err| {
            // fails if CWD doesnt exist or insufficient permissions
            anyhow!("could not yoink current directory: {err}")
        })?,
    };

    // yoink the files using cli params
    yoink::pull(path, cli.recursive)
}
