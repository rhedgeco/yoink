use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bytes {
    path: PathBuf,
}

impl Bytes {
    pub fn pull(&self, mut target: impl io::Write) -> Option<usize> {
        match fs::read(&self.path) {
            Ok(bytes) => {
                if let Err(err) = target.write_all(&bytes) {
                    eprintln!("Failed to write bytes to file: {err}");
                    return None;
                }

                return Some(bytes.len());
            }
            Err(err) => {
                eprintln!("Failed to read '{}': {err}", self.path.display());
                return None;
            }
        }
    }

    pub fn push(&self, mut source: impl io::Read) {
        let mut buffer = Vec::new();
        if let Err(err) = source.read_to_end(&mut buffer) {
            eprintln!(
                "Failed to read source file for '{}': {err}",
                self.path.display()
            );
            return;
        };

        let Some(parent) = self.path.parent() else {
            eprintln!("Could not get parent of '{}'", self.path.display());
            return;
        };

        if let Err(err) = fs::create_dir_all(parent) {
            eprintln!(
                "Failed to create parents directories for '{}': {err}",
                self.path.display()
            );
            return;
        };

        let mut target = match File::options().create(true).write(true).open(&self.path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!(
                    "Failed to open '{}' for writing: {err}",
                    self.path.display()
                );
                return;
            }
        };

        if let Err(err) = target.write_all(&buffer) {
            eprintln!("Failed to write to '{}': {err}", self.path.display());
            return;
        }

        if let Err(err) = target.set_len(buffer.len() as u64) {
            eprintln!(
                "Failed to truncate '{}' after writing: {err}",
                self.path.display()
            );
        }
    }
}
