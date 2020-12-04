use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::error::{AocError, Result};

pub fn load_lines(file: &str) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    for line in BufReader::new(File::open(Path::new(file))?).lines() {
        lines.push(line?);
    }

    Ok(lines)
}

pub fn load_input(day: &str) -> Result<Vec<String>> {
    // examples/003_toboggan-trajectory/input
    let examples_dir = Path::new("examples");
    for entry in fs::read_dir(examples_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && entry.file_name().into_string()?.starts_with(day) {
            if let Some(file) = path.join("input").to_str() {
                return load_lines(file);
            }
        }
    }
    Err(AocError::InputError(
        format!("Could not find or load input for {}", day).to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: write tests - MCL - 2020-12-03
}
