use std::process::Command;
use anyhow::Result;

use crate::error::Error;

pub fn cwebp(in_file: &str, out_file: &str) -> Result<()> {
    let cmd = Command::new("cwebp")
        .args(vec![in_file, out_file])
        .output()?;

    if !cmd.status.success() {
        return Err(todo!{})
    }

    Ok(())
}
