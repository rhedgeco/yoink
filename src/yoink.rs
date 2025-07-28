use std::{
    ffi::OsStr,
    fs::{self, File},
    path::Path,
};

use serde::Deserialize;

use crate::style::Bytes;

pub fn pull(path: impl AsRef<Path>, recursive: bool) {
    let path = path.as_ref();
    dispatch_file_or_dir(path, || pull_file(path), || pull_dir(path, recursive));
}

pub fn push(path: impl AsRef<Path>, recursive: bool) {
    let path = path.as_ref();
    dispatch_file_or_dir(path, || push_file(path), || push_dir(path, recursive));
}

fn pull_file(path: &Path) {
    let Some(config) = load_config(path) else {
        return;
    };

    let target_path = path.with_extension("");
    let mut target = match File::options().write(true).create(true).open(&target_path) {
        Ok(target) => target,
        Err(err) => {
            eprintln!("Failed to create '{}': {err}", target_path.display());
            return;
        }
    };

    if !set_current_dir_relative_to_file(path) {
        return;
    }

    // format and write the information to the target
    let length = match &config.target.style {
        Style::Bytes(bytes) => bytes.pull(&mut target),
    };

    // if there is no written length,
    // then there was a failure and we should return
    let Some(length) = length else {
        return;
    };

    // otherwise truncate the file to the length of the written bytes
    if let Err(err) = target.set_len(length as u64) {
        eprintln!(
            "Failed to truncate '{}' target after writing: {err}",
            path.display()
        );
    };

    println!("Pulled '{}'", path.display());
}

fn push_file(path: &Path) {
    let Some(config) = load_config(path) else {
        return;
    };

    let source_path = path.with_extension("");
    let mut source = match File::options().read(true).open(&source_path) {
        Ok(target) => target,
        Err(err) => {
            eprintln!(
                "Failed to open '{}' for reading: {err}",
                source_path.display()
            );
            return;
        }
    };

    if !set_current_dir_relative_to_file(path) {
        return;
    }

    // push the source data using the specified config style
    match &config.target.style {
        Style::Bytes(bytes) => bytes.push(&mut source),
    }

    println!("Pushed '{}'", path.display());
}

fn pull_dir(path: &Path, recursive: bool) {
    iterate_yoinkfiles(path, recursive, &|path| pull_file(&path));
}

fn push_dir(path: &Path, recursive: bool) {
    iterate_yoinkfiles(path, recursive, &|path| push_file(&path));
}

fn iterate_yoinkfiles(path: &Path, recursive: bool, f: &impl Fn(&Path)) {
    let read_dir = match fs::read_dir(path) {
        Ok(read_dir) => read_dir,
        Err(err) => {
            eprintln!("Failed to read dir '{}': {err}", path.display());
            return;
        }
    };

    for entry in read_dir {
        match entry {
            Ok(entry) => {
                let subpath = entry.path();
                if subpath == path {
                    continue;
                }

                if subpath.is_dir() && recursive {
                    iterate_yoinkfiles(&subpath, recursive, f);
                }

                if subpath.is_file() && subpath.extension() == Some(OsStr::new("yoink")) {
                    f(&subpath);
                }
            }
            Err(err) => {
                eprintln!("Failed to read child of '{}', {err}", path.display());
            }
        }
    }
}

fn dispatch_file_or_dir(path: &Path, file: impl FnOnce(), dir: impl FnOnce()) {
    if !path.exists() {
        eprintln!("'{}' does not exist", path.display());
        return;
    }

    if path.is_file() {
        file();
        return;
    }

    if path.is_dir() {
        dir();
        return;
    }

    eprintln!("'{}' is not a file or directory", path.display())
}

#[derive(Deserialize)]
struct Config {
    target: Target,
}

#[derive(Deserialize)]
struct Target {
    #[serde(flatten)]
    style: Style,
}

#[derive(Deserialize)]
#[serde(tag = "style", rename_all = "snake_case")]
enum Style {
    Bytes(Bytes),
}

fn load_config(path: &Path) -> Option<Config> {
    // ensure the file contains the correct 'yoink' extension
    if path.extension() != Some(OsStr::new("yoink")) {
        eprintln!("Failed to load '{}': no '.yoink' extension", path.display());
        return None;
    }

    // load the yoink file contents
    match fs::read_to_string(path) {
        Err(err) => {
            eprintln!("Failed to read '{}': {err}", path.display());
            return None;
        }
        Ok(content) => match toml::from_str::<Config>(&content) {
            Ok(config) => Some(config),
            Err(err) => {
                eprintln!("Failed to parse '{}':\n{err}", path.display());
                return None;
            }
        },
    }
}

fn set_current_dir_relative_to_file(path: &Path) -> bool {
    // get the parent directory of the target path
    let Some(parent_dir) = path.parent() else {
        eprintln!("Failed to get parent directory of '{}'", path.display());
        return false;
    };

    // set the parent directory as the current working directory
    // this is so that future operations are relative to the yoink file
    if let Err(err) = std::env::set_current_dir(&parent_dir) {
        eprintln!(
            "Failed to set relative directory to '{}': {err}",
            parent_dir.display()
        );
        return false;
    }

    true
}
