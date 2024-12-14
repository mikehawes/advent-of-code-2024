use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

pub fn input_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let full_path = file_in_src(path);
    read_to_string(full_path)
}

pub fn file_in_src<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    full_path.push("src");
    full_path.push(path);
    full_path
}
