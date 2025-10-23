use std::path::PathBuf;

use anyhow::{anyhow, bail};
use clap::Parser;
use yoink::runner;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// path to a `yoink` file, or a directory containing `yoink` files
    pub path: PathBuf,

    /// yoink subdirectories when a directory path is specified
    #[arg(short, long)]
    pub recursive: bool,
}

fn main() -> anyhow::Result<()> {
    // parse the command line args
    let cli = Cli::parse();

    // ensure the target path exists
    if !cli.path.exists() {
        bail!("'{}' does not exist", cli.path.display());
    }

    // if its a file, yoink it
    if cli.path.is_file() {
        return runner::yoink_file(&cli.path).map_err(|err| {
            let path_display = cli.path.display();
            anyhow!("Failed to yoink '{path_display}' -> {err}")
        });
    }

    // if its a directory, yoink it
    if cli.path.is_dir() {
        return runner::yoink_dir(&cli.path, cli.recursive).map_err(|err| {
            let path_display = cli.path.display();
            anyhow!("Failed to yoink '{path_display}' -> {err}")
        });
    }

    // if the path is neither a file or directory, throw an error
    bail!("'{}' is not a file or directory", cli.path.display());
}
