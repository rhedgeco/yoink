use std::{env, fs, path::Path};

use anyhow::bail;

use crate::Config;

pub trait Yoink {
    fn pull(&mut self, store: Option<&[u8]>) -> anyhow::Result<Box<[u8]>>;
}

pub fn pull(path: impl AsRef<Path>, recursive: bool) -> anyhow::Result<()> {
    let path = path.as_ref();

    if path.is_file() {
        return pull_file(path);
    }

    if path.is_dir() {
        return pull_dir(path, recursive);
    }

    bail!(
        "Failed to pull '{}': not a file or directory",
        path.display()
    );
}

fn pull_file(path: &Path) -> anyhow::Result<()> {
    assert!(path.is_file());
    assert!(utils::has_yoink_extension(path));

    // read and parse the yoink file config
    let content = fs::read_to_string(path)?;
    let mut config: Config = toml::from_str(&content)?;

    // load the current store content
    let store_path = path.with_extension("");
    let store_content = match store_path.exists() {
        true => Some(utils::store_err(fs::read(&store_path))?),
        false => None,
    };

    // set the current working directory so relative paths resolve correctly
    let working_directory = env::current_dir().expect("valid working directory");
    let path_parent = path.parent().expect("valid file parent");
    env::set_current_dir(path_parent).expect("valid parent");

    // generate the new store content
    let store_bytes = store_content.as_ref().map(Vec::as_slice);
    let new_store_content = config.target.pull(store_bytes)?;

    // reset working directory and write the new store file
    env::set_current_dir(working_directory).expect("valid working directory");
    utils::store_err(fs::write(store_path, new_store_content))?;

    // print the pull success and return ok
    println!("pulled '{}'", path.display());
    Ok(())
}

fn pull_dir(path: &Path, recursive: bool) -> anyhow::Result<()> {
    assert!(path.is_dir());

    // try to pull every file in the directory
    for entry in fs::read_dir(path)? {
        // try to read the path from the current entry
        let sub_path = match entry {
            Ok(entry) => entry.path(),
            Err(err) => {
                eprintln!("Failed to read entry in '{}': {err}", path.display());
                continue;
            }
        };

        // if the path is a directory and recursive is true, try to pull it
        if sub_path.is_dir() && recursive {
            let Err(err) = pull_dir(&sub_path, recursive) else {
                continue;
            };

            eprintln!("Failed to pull dir '{}': {err}", sub_path.display());
            continue;
        }

        // if the path is a file with a yoink extension, try to pull it
        if sub_path.is_file() && utils::has_yoink_extension(&sub_path) {
            let Err(err) = pull_file(&sub_path) else {
                continue;
            };

            eprintln!("Failed to pull '{}': {err}", sub_path.display());
            continue;
        }
    }

    Ok(())
}

mod utils {
    use std::{ffi::OsStr, fmt::Display, path::Path};

    use anyhow::anyhow;

    pub fn yoink_str() -> &'static OsStr {
        OsStr::new("yoink")
    }

    pub fn has_yoink_extension(path: impl AsRef<Path>) -> bool {
        path.as_ref().extension() == Some(yoink_str())
    }

    pub fn store_err<T, E: Display>(result: Result<T, E>) -> anyhow::Result<T> {
        result.map_err(|err| anyhow!("local store error: {err}"))
    }
}
