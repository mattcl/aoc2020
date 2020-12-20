use crate::error::{AocError, Result};
use std::collections::{HashMap, HashSet};

pub fn input_map(input: &[String]) -> Result<HashMap<usize, &str>> {
    let mut map = HashMap::new();
    for line in input.iter() {
        let mut parts = line.split(": ");
        if let Some(id) = parts.next() {
            if let Some(def) = parts.next() {
                map.insert(id.parse::<usize>()?, def);
            }
        }
    }
    Ok(map)
}

pub fn get_matching_messages(input: &[String]) -> Result<HashSet<String>> {
    let mut parts = input.split(|line| line.is_empty());
    if let Some(rules) = parts.next() {
        let map = input_map(rules)?;
        let ruleset = Ruleset::from_input_map(&map)?;
        if let Some(messages) = parts.next() {
            return Ok(messages
                .iter()
                .cloned()
                .filter(|line| ruleset.check(0, line))
                .collect::<HashSet<String>>());
        }
        return Err(AocError::InvalidInput("Input missing messages".to_string()));
    }

    Err(AocError::InvalidInput("Input missing rules".to_string()))
}

pub fn get_matching_messages_b(input: &[String]) -> Result<HashSet<String>> {
    let mut parts = input.split(|line| line.is_empty());
    if let Some(rules) = parts.next() {
        let map = input_map(rules)?;
        let ruleset = Ruleset::from_input_map(&map)?;
        if let Some(messages) = parts.next() {
            return Ok(messages
                .iter()
                .cloned()
                .filter(|line| ruleset.check_b(0, line))
                .collect::<HashSet<String>>());
        }
        return Err(AocError::InvalidInput("Input missing messages".to_string()));
    }

    Err(AocError::InvalidInput("Input missing rules".to_string()))
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Val(char),
    Rule(usize),
}

impl Token {
    pub fn check(&self, start: &mut usize, input: &[char], rules: &HashMap<usize, Rule>) -> bool {
        if *start < input.len() {
            let cur = *start;
            match self {
                Self::Val(ch) => {
                    if input[*start] == *ch {
                        *start += 1;
                        return true;
                    }
                    return false;
                }
                Self::Rule(rule_id) => {
                    if let Some(rule) = rules.get(rule_id) {
                        if rule.check(start, input, rules) {
                            return true;
                        }
                        *start = cur
                    }
                }
            }
        }

        false
    }

    pub fn check_b(
        &self,
        start: &HashSet<usize>,
        input: &[char],
        rules: &HashMap<usize, Rule>,
    ) -> HashSet<usize> {
        match self {
            Self::Val(ch) => start
                .iter()
                .filter(|i| match input.get(**i) {
                    Some(c) => *ch == *c,
                    _ => false,
                })
                .map(|i| i + 1)
                .collect::<HashSet<usize>>(),
            Self::Rule(rule_id) => {
                if let Some(rule) = rules.get(rule_id) {
                    rule.check_b(start, input, rules)
                } else {
                    HashSet::new()
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group(Vec<Token>);

impl Group {
    pub fn new() -> Self {
        Group(Vec::new())
    }

    pub fn check(&self, start: &mut usize, input: &[char], rules: &HashMap<usize, Rule>) -> bool {
        let cur = *start;
        for token in self.0.iter() {
            if !token.check(start, input, rules) {
                *start = cur;
                return false;
            }
        }
        true
    }

    pub fn check_b(
        &self,
        start: &HashSet<usize>,
        input: &[char],
        rules: &HashMap<usize, Rule>,
    ) -> HashSet<usize> {
        let mut start = start.clone();
        for (index, token) in self.0.iter().enumerate() {
            let cur = token.check_b(&start, input, rules);
            if cur.is_empty() {
                return cur;
            }

            if index == self.0.len() - 1 {
                return cur;
            }
            start = cur;
        }
        HashSet::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    id: usize,
    groups: Vec<Group>,
}

impl Rule {
    pub fn parse(id: usize, def: &str) -> Result<Self> {
        let mut groups = Vec::new();

        for group_def in def.split(" | ") {
            let mut group = Group::new();

            for part in group_def.split(' ') {
                if let Ok(num) = part.parse::<usize>() {
                    group.0.push(Token::Rule(num));
                } else {
                    // "<ch>" => "a"
                    if let Some(ch) = part.chars().skip(1).next() {
                        group.0.push(Token::Val(ch));
                    } else {
                        return Err(AocError::InvalidInput(format!(
                            "Cannot parse {}: '{}'",
                            id, def
                        )));
                    }
                }
            }

            groups.push(group);
        }

        Ok(Rule {
            id: id,
            groups: groups,
        })
    }

    pub fn check(&self, start: &mut usize, input: &[char], rules: &HashMap<usize, Rule>) -> bool {
        let cur = *start;
        for group in self.groups.iter() {
            if group.check(start, input, rules) {
                return true;
            }
            *start = cur;
        }
        false
    }

    pub fn check_b(
        &self,
        start: &HashSet<usize>,
        input: &[char],
        rules: &HashMap<usize, Rule>,
    ) -> HashSet<usize> {
        self.groups.iter().fold(HashSet::new(), |acc, group| {
            acc.union(&group.check_b(start, input, rules))
                .cloned()
                .collect()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ruleset {
    rules: HashMap<usize, Rule>,
}

impl Ruleset {
    pub fn from_input_map(map: &HashMap<usize, &str>) -> Result<Self> {
        let mut rules: HashMap<usize, Rule> = HashMap::new();

        for (id, def) in map {
            rules.insert(*id, Rule::parse(*id, def)?);
        }

        Ok(Ruleset { rules })
    }

    pub fn get(&self, id: usize) -> Option<&Rule> {
        self.rules.get(&id)
    }

    pub fn check(&self, id: usize, input: &str) -> bool {
        if let Some(rule) = self.get(id) {
            let input = input.chars().collect::<Vec<char>>();
            let mut index = 0;
            if rule.check(&mut index, &input, &self.rules) {
                return index == input.len();
            }
        }
        false
    }

    pub fn check_b(&self, id: usize, input: &str) -> bool {
        if let Some(rule) = self.get(id) {
            let input = input.chars().collect::<Vec<char>>();
            let endpoints = rule.check_b(
                &vec![0].into_iter().collect::<HashSet<usize>>(),
                &input,
                &self.rules,
            );
            return endpoints.contains(&input.len());
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_input;

    #[test]
    fn rule_matching_example_1() {
        let input = test_input(
            "
                0: 1 2
                1: \"a\"
                2: 1 3 | 3 1
                3: \"b\"
            ",
        );

        let ruleset = Ruleset::from_input_map(&input_map(&input).unwrap()).unwrap();

        assert!(ruleset.check(0, "aab"));
        assert!(ruleset.check(0, "aba"));
        assert!(!ruleset.check(0, "xxx"));
        assert!(!ruleset.check(0, "aa"));
        assert!(!ruleset.check(0, "ab"));

        assert!(ruleset.check_b(0, "aab"));
        assert!(ruleset.check_b(0, "aba"));
        assert!(!ruleset.check_b(0, "xxx"));
        assert!(!ruleset.check_b(0, "aa"));
        assert!(!ruleset.check_b(0, "ab"));
    }

    #[test]
    fn rule_matching_example_2() {
        let input = test_input(
            "
                0: 4 1 5
                1: 2 3 | 3 2
                2: 4 4 | 5 5
                3: 4 5 | 5 4
                4: \"a\"
                5: \"b\"
            ",
        );

        /*
         * ababbb
         * abbbab
         *
         * bababa
         * aaabbb
         * aaaabbb
         */

        let ruleset = Ruleset::from_input_map(&input_map(&input).unwrap()).unwrap();

        assert!(ruleset.check(0, "ababbb"));
        assert!(ruleset.check(0, "abbbab"));
        assert!(!ruleset.check(0, "bababa"));
        assert!(!ruleset.check(0, "aaabbb"));
        assert!(!ruleset.check(0, "aaaabbb"));
    }

    #[test]
    fn rules_with_loops() {
        let input = test_input(
            "
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: \"a\"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: \"b\"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        ",
        );

        assert_eq!(get_matching_messages(&input).unwrap().len(), 3);

        let input = test_input(
            "
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: \"a\"
            11: 42 31 | 42 11 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: \"b\"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42 | 42 8
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        ",
        );

        let expected = vec![
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();

        let matching = get_matching_messages_b(&input).unwrap();
        println!("{:#?}", expected.difference(&matching));
        println!("{:#?}", matching);

        assert_eq!(matching.len(), 12);
    }

    #[test]
    fn foo() {
        let input = test_input(
            "
            0: \"a\" 0 | \"b\"

            ab
            aab
            aaab
            aaaab
            baaaa
        ",
        );

        let matching = get_matching_messages_b(&input).unwrap();
        println!("{:#?}", matching);

        assert_eq!(matching.len(), 4);

        // 11: 42 31 | 42 11 31
        let input = test_input(
            "
            0: \"b\" \"a\" | \"b\" 0 \"a\"

            ba
            bbaa
            bbbbaaaa
        ",
        );

        let matching = get_matching_messages_b(&input).unwrap();
        println!("{:#?}", matching);

        assert_eq!(matching.len(), 3);

        let input = test_input(
            "
            0: \"a\" | \"a\" 0

            a
            aa
            aaa
            aaaa
            abaaa
        ",
        );

        let matching = get_matching_messages_b(&input).unwrap();
        println!("{:#?}", matching);

        assert_eq!(matching.len(), 4);
    }
}
