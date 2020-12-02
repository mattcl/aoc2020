use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::error::Result;

pub fn load_lines(file: &str) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    for line in BufReader::new(File::open(Path::new(file))?).lines() {
        lines.push(line?);
    }

    Ok(lines)
}
