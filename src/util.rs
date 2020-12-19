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
    //
    // examples/003_toboggan-trajectory/input
    //
    load_named_input(day, "input")
}

pub fn load_named_input(day: &str, name: &str) -> Result<Vec<String>> {
    //
    // examples/003_toboggan-trajectory/<name>
    //
    let examples_dir = Path::new("examples");
    for entry in fs::read_dir(examples_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && entry.file_name().into_string()?.starts_with(day) {
            if let Some(file) = path.join(name).to_str() {
                return load_lines(file);
            }
        }
    }
    Err(AocError::InputError(format!(
        "Could not find or load input for {}: '{}'",
        day,
        name
    )))
}

pub fn test_input(input: &str) -> Vec<String> {
    // TODO: figure out if trim is the right thing to do - MCL - 2020-12-10
    input
        .trim()
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_conversion() {
        let expected = vec![
            "abcd".to_string(),
            "".to_string(),
            "".to_string(),
            "efgh".to_string(),
            "ijkl".to_string(),
            "mnop".to_string(),
            "".to_string(),
            "qrs".to_string(),
        ];

        let input = "
            abcd


            efgh
            ijkl
            mnop

            qrs
        ";
        assert_eq!(test_input(input), expected);
    }
}
