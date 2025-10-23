use std::{ffi::OsStr, fs, path::Path};

use anyhow::bail;

use crate::config::{Config, Style};

pub mod bytes;

pub fn yoink_file(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    assert!(path.is_file());
    assert!(path.extension() == Some(OsStr::new("yoink")));

    // read and parse the yoinkfile
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;

    // get the parent directory of the yoinkfile
    let parent_dir = path.parent().expect("valid parent");

    // get the current working directory
    let working_dir = std::env::current_dir()?;

    // set the current working directory
    // this is so that different yoink styles can use relative paths
    std::env::set_current_dir(parent_dir)?;

    // then yoink the bytes using the correct style
    let mut bytes = Vec::new();
    match config.target.style {
        Style::Bytes { path } => bytes::yoink(path, &mut bytes)?,
    };

    // reset the working directory after finished writing
    std::env::set_current_dir(working_dir)?;

    // write the content to the target file
    let target_path = path.with_extension("");
    fs::write(target_path, bytes)?;

    Ok(())
}

pub fn yoink_dir(path: impl AsRef<Path>, recursive: bool) -> anyhow::Result<()> {
    let path = path.as_ref();
    assert!(path.is_dir());

    // create a flag to track if any yoinks failed
    let mut failed = false;

    // search all the item in the directory for yoinkfiles
    for entry in fs::read_dir(path)? {
        // try to get the path from the entry
        let sub_path = match entry {
            Ok(entry) => entry.path(),
            Err(err) => {
                eprintln!("Failed to read entry in '{}': {err}", path.display());
                failed = true;
                continue;
            }
        };

        // yoink any subdirectories if the recursive flag is set
        if sub_path.is_dir() && recursive {
            let Err(err) = yoink_dir(&sub_path, recursive) else {
                continue;
            };

            eprintln!("Error: Failed to yoink '{}' -> {err}", sub_path.display());
            failed = true;
            continue;
        }

        // yoink any files if they have the yoink extension
        if sub_path.is_file() && sub_path.extension() == Some(OsStr::new("yoink")) {
            let Err(err) = yoink_file(&sub_path) else {
                continue;
            };

            eprintln!("Error: Failed to yoink '{}' -> {err}", sub_path.display());
            failed = true;
            continue;
        }
    }

    // if there was a failure, return a failure from the whole function
    if failed {
        bail!("one or more paths could not be yoinked");
    }

    Ok(())
}
