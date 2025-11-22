use std::fs;
use std::io;

pub fn read_file_content(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}