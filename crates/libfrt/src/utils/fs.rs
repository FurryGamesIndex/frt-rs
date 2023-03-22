use std::path::Path;

use anyhow::Result;

use crate::error::{Error, ErrorKind};

pub fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(src: U, dst: V) -> Result<()> {
    std::fs::create_dir_all(&dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let target = dst.as_ref().join(entry.file_name());

        if file_type.is_dir() {
            copy_dir(entry.path(), target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }

    Ok(())
}

pub fn get_mtime<U: AsRef<Path>>(f: U) -> Result<u64> {
    Ok(std::fs::metadata(f)?
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs())
}

pub fn make_dir<U: AsRef<Path>>(p: U) -> Result<()> {
    let p = p.as_ref();

    if !p.exists() {
        info!("Make directory: '{}'", p.display());
        std::fs::create_dir_all(p)?;
    }

    Ok(())
}

pub fn ensure_dir<U: AsRef<Path>>(p: U) -> Result<()> {
    let p = p.as_ref();
    let p = p
        .parent()
        .ok_or_else(|| crate::err!(NotExist, "File not exist: {}", p.display()))?;

    make_dir(p)?;

    Ok(())
}
