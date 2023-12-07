use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_from_files(file_path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().collect::<Result<_, _>>()?)
}