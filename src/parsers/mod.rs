use std::{path::PathBuf, str::FromStr};

pub fn is_directory(s: &str) -> Result<PathBuf, String> {
    let dir: PathBuf = PathBuf::from_str(s).map_err(|_| format!("`{s}` is not a valid path"))?;

    if dir.is_dir() {
        Ok(dir as PathBuf)
    } else {
        Err(format!("`{s}` is not a existing directory"))
    }
}
