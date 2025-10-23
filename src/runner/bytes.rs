use std::{fs, io, path::Path};

use anyhow::anyhow;

pub fn yoink(path: impl AsRef<Path>, mut write: impl io::Write) -> anyhow::Result<()> {
    let path = path.as_ref();

    // read the bytes directly from the target path
    let bytes = fs::read(path).map_err(|err| {
        let path_display = path.display();
        anyhow!("'{path_display}': {err}")
    })?;

    // then write the bytes directly
    write.write(&bytes)?;
    Ok(())
}
