use std::{fs, io, path::Path};

pub fn yoink(path: impl AsRef<Path>, mut write: impl io::Write) -> anyhow::Result<()> {
    let path = path.as_ref();
    let bytes = fs::read(path)?;
    write.write(&bytes)?;
    Ok(())
}
