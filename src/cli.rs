use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
    /// Pull '*.yoink' file content from the system to the local store
    Pull,
    /// Push local '*.yoink' file content to the target system locations
    Push,
}

#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Recurse subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Target file or directory to be yoinked
    #[arg(short, long)]
    pub target: Option<PathBuf>,
}
