use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

pub fn input_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    full_path.push("src");
    full_path.push(path);
    read_to_string(full_path)
}
