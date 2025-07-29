use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// path to a `yoink` file or a directory containing `yoink` files
    path: PathBuf,

    /// yoink subdirectories when a directory path is specified
    #[arg(short, long)]
    recursive: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// pull information from the yoink target
    Pull,

    /// push information to the yoink target
    Push,
}

fn main() {
    // parse command line arguments
    let cli = Cli::parse();

    // build the path into an absolute path
    let absolute = match cli.path.canonicalize() {
        Ok(absolute) => absolute,
        Err(err) => {
            eprintln!("Failed to read path '{}': {err}", cli.path.display());
            return;
        }
    };

    // dispatch yoink based on chosen command
    // if no command was chosen, default to the 'pull' command
    match cli.command {
        None | Some(Command::Pull) => yoink::pull(absolute, cli.recursive),
        Some(Command::Push) => yoink::push(absolute, cli.recursive),
    }
}
