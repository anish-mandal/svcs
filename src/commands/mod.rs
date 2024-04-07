use std::{fs, path::PathBuf};

pub fn init(direc: &Option<PathBuf>) {
    let mut dir: PathBuf = match direc {
        Some(d) => d.clone().canonicalize().unwrap(),
        None => PathBuf::from(".").canonicalize().unwrap(),
    };

    dir.push(".svcs/master");
    let directories: [&str; 3] = ["commits", "blobs", "structures"];

    for d in directories {
        fs::create_dir_all(dir.as_path().join(d)).unwrap();
    }
}
