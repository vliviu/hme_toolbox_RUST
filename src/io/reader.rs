use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn read_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut content = String::new();
    BufReader::new(file).read_to_string(&mut content)?;
    Ok(content)
}
