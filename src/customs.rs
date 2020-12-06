use crate::error::{AocError, Result};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Group {
    size: usize,
    answers: HashMap<char, usize>,
}

impl Group {
    pub fn from_input(input: &[String]) -> Result<Vec<Self>> {
        input
            .split(|line| line.is_empty())
            .map(|group| Self::new(group))
            .collect::<Result<Vec<Self>>>()
    }

    pub fn new(data: &[String]) -> Result<Self> {
        let mut answers = HashMap::new();
        for line in data {
            for ch in line.to_lowercase().chars() {
                if ch.is_alphabetic() {
                    if let Some(count) = answers.get_mut(&ch) {
                        *count += 1;
                    } else {
                        answers.insert(ch, 1);
                    }
                } else if !ch.is_whitespace() {
                    return Err(AocError::InvalidAnswers(format!("{:?}", data)));
                }
            }
        }

        Ok(Group {
            size: data.len(),
            answers: answers,
        })
    }

    pub fn unique_answers(&self) -> usize {
        self.answers.len()
    }

    pub fn collective_answers(&self) -> usize {
        self.answers.values().filter(|v| **v == self.size).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_input() {
        let input = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

        let res = Group::from_input(&input).expect("Unexpected errors in input");
        assert_eq!(res.len(), 5);
    }

    #[test]
    fn construction() {
        let data = vec!["abd", "abx"]
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let res = Group::new(&data);
        assert!(res.is_ok());

        let data = vec!["abd?a", "abx"]
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let res = Group::new(&data);
        assert!(res.is_err());
    }

    #[test]
    fn unique_answers() {
        let data = vec!["abcda", "abx"]
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let res = Group::new(&data).expect("data was supposed to be valid");
        assert_eq!(res.unique_answers(), 5);

        let data = vec!["abda", "abx", "x"]
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let res = Group::new(&data).expect("data was supposed to be valid");
        assert_eq!(res.unique_answers(), 4);
    }

    #[test]
    fn collective_answers() {
        let data = vec!["abcd", "abx"]
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let res = Group::new(&data).expect("data was supposed to be valid");
        assert_eq!(res.collective_answers(), 2);

        let data = vec!["abd", "abx", "x"]
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let res = Group::new(&data).expect("data was supposed to be valid");
        assert_eq!(res.collective_answers(), 0);
    }
}
