use std::collections::HashMap;

use crate::error::{AocError, Result};

pub enum PolicyType {
    Count,
    Position,
}

pub struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl Policy {
    fn new(def: &str) -> Result<Self> {
        let parts: Vec<&str> = def.split(" ").collect();

        if parts.len() == 2 {
            let range: Vec<&str> = parts[0].split("-").collect();
            if range.len() == 2 {
                if let Some(letter) = parts[1].chars().collect::<Vec<char>>().first() {
                    return Ok(Policy {
                        min: range[0].parse()?,
                        max: range[1].parse()?,
                        letter: *letter,
                    });
                }
            }
        }

        return Err(AocError::PolicyDefinitionError(def.to_string()));
    }

    fn allowed(&self, password: &str, policy_type: &PolicyType) -> bool {
        match policy_type {
            PolicyType::Count => self.count_check(password),
            PolicyType::Position => self.position_check(password),
        }
    }

    fn count_check(&self, password: &str) -> bool {
        let count = password
            .to_lowercase()
            .chars()
            .filter(|ch| *ch == self.letter)
            .collect::<Vec<char>>()
            .len();

        count >= self.min && count <= self.max
    }

    fn position_check(&self, password: &str) -> bool {
        let count = password
            .to_lowercase()
            .chars()
            .enumerate()
            .filter(|(index, ch)| {
                *ch == self.letter && (index + 1 == self.min || index + 1 == self.max)
            })
            .collect::<Vec<(usize, char)>>()
            .len();

        count == 1
    }
}

pub fn count_valid_passwords(input: &Vec<String>, policy_type: &PolicyType) -> Result<usize> {
    let mut policies: HashMap<&str, Policy> = HashMap::new();
    let mut count = 0;
    for candidate in input {
        let parts = candidate.split(": ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(AocError::PasswordDefinitionError(candidate.to_string()));
        }

        if !policies.contains_key(parts[0]) {
            policies.insert(parts[0], Policy::new(parts[0])?);
        }

        if let Some(policy) = policies.get(parts[0]) {
            if policy.allowed(parts[1], policy_type) {
                count += 1;
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_valid_password() {
        let input = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];

        let res = count_valid_passwords(&input, &PolicyType::Count);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 2);

        let res = count_valid_passwords(&input, &PolicyType::Position);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);
    }

    mod policy {
        use super::super::*;

        #[test]
        fn valid_policies() {
            let p = Policy::new("1-3 a");
            assert!(p.is_ok());

            let p = p.unwrap();

            assert_eq!(p.min, 1);
            assert_eq!(p.max, 3);
            assert_eq!(p.letter, 'a');

            let p = Policy::new("2-9 c");
            assert!(p.is_ok());

            let p = p.unwrap();

            assert_eq!(p.min, 2);
            assert_eq!(p.max, 9);
            assert_eq!(p.letter, 'c');
        }

        #[test]
        fn invalid_policies() {
            let p = Policy::new("1-3");
            assert!(p.is_err());

            let p = Policy::new("1-3 ");
            assert!(p.is_err());

            let p = Policy::new("1-3 c j");
            assert!(p.is_err());

            let p = Policy::new("1 c");
            assert!(p.is_err());

            let p = Policy::new("a-4 c");
            assert!(p.is_err());
        }

        #[test]
        fn count_checking() {
            // passes
            let p = Policy::new("1-3 a").unwrap();
            assert!(p.count_check("abcde"));
            assert!(p.count_check("abcade"));
            assert!(p.count_check("aabcade"));

            let p = Policy::new("2-9 c").unwrap();
            assert!(p.count_check("accde"));
            assert!(p.count_check("abccccade"));
            assert!(p.count_check("ccccccccc"));

            // fails
            let p = Policy::new("1-3 a").unwrap();
            assert!(!p.count_check("aaabcade"));
            assert!(!p.count_check("bcde"));

            let p = Policy::new("2-9 c").unwrap();
            assert!(!p.count_check("acde"));
            assert!(!p.count_check("cccccccccc"));
        }

        #[test]
        fn position_checking() {
            // passes
            let p = Policy::new("1-3 a").unwrap();
            assert!(p.position_check("abcde"));
            assert!(p.position_check("bbade"));
            assert!(p.position_check("aabcade"));

            let p = Policy::new("2-9 c").unwrap();
            assert!(p.position_check("accde"));
            assert!(p.position_check("abccccadc"));
            assert!(p.position_check("cccccccc"));

            // fails
            let p = Policy::new("1-3 a").unwrap();
            assert!(!p.position_check("aaabcade"));
            assert!(!p.position_check("bcde"));

            let p = Policy::new("2-9 c").unwrap();
            assert!(!p.position_check("abdec"));
            assert!(!p.position_check("cccccccccc"));
        }
    }
}
