use std::{env::current_dir, path::PathBuf};

use crate::error::ProcessErr;

pub fn get_tmp_dir() -> Result<PathBuf, ProcessErr> {
    let tmp_dir = current_dir()?.join("tmp");
    Ok(tmp_dir)
}
