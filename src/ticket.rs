use crate::error::{AocError, Result};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Range(usize, usize);

impl Range {
    pub fn includes(&self, val: usize) -> bool {
        val >= self.0 && val <= self.1
    }
}

impl FromStr for Range {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split('-');

        if let Some(lower) = parts.next() {
            if let Some(upper) = parts.next() {
                if let Some(_) = parts.next() {
                    return Err(AocError::InvalidInput(format!(
                        "Cannot make range from '{}'",
                        s
                    )));
                }

                let range = Range(lower.parse::<usize>()?, upper.parse::<usize>()?);

                if range.0 > range.1 {
                    return Err(AocError::InvalidInput(format!(
                        "Cannot make range from (low is higher than high) '{}'",
                        s
                    )));
                }

                return Ok(range);
            }
        }
        Err(AocError::InvalidInput(format!(
            "Cannot make range from '{}'",
            s
        )))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Rule {
    pub name: String,
    pub ranges: Vec<Range>,
}

impl Rule {
    pub fn new(name: &str, ranges: &Vec<Range>) -> Self {
        Rule {
            name: name.to_string(),
            ranges: ranges.clone(),
        }
    }

    pub fn includes(&self, val: usize) -> bool {
        for r in &self.ranges {
            if r.includes(val) {
                return true;
            }
        }

        false
    }
}

impl FromStr for Rule {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(": ");

        if let Some(name) = parts.next() {
            if let Some(ranges) = parts.next() {
                let ranges = ranges
                    .split(" or ")
                    .map(|r| Range::from_str(r))
                    .collect::<Result<Vec<Range>>>()?;

                if ranges.is_empty() {
                    return Err(AocError::InvalidInput(format!(
                        "Cannot make rule from '{}'",
                        s
                    )));
                }

                return Ok(Rule::new(name, &ranges));
            }
        }

        Err(AocError::InvalidInput(format!(
            "Cannot make rule from '{}'",
            s
        )))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Ticket {
    pub values: Vec<usize>,
    pub is_valid: bool,
}

impl Ticket {
    pub fn new(values: &Vec<usize>) -> Self {
        Ticket {
            values: values.clone(),
            is_valid: false,
        }
    }
}

impl FromStr for Ticket {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        let vals = s
            .split(',')
            .map(|v| v.parse::<usize>())
            .collect::<std::result::Result<Vec<usize>, std::num::ParseIntError>>()?;

        Ok(Ticket::new(&vals))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TicketValidator {
    pub rules: Vec<Rule>,
}

impl TicketValidator {
    pub fn from_input(input: &[String]) -> Result<Self> {
        Ok(Self::new(
            &input
                .iter()
                .map(|line| Rule::from_str(line))
                .collect::<Result<Vec<Rule>>>()?,
        ))
    }

    pub fn new(rules: &Vec<Rule>) -> Self {
        TicketValidator {
            rules: rules.clone(),
        }
    }

    pub fn validate(&self, ticket: &mut Ticket) -> Option<Vec<usize>> {
        let mut invalid = Vec::new();
        for v in ticket.values.iter() {
            let mut found = false;
            for rule in &self.rules {
                if rule.includes(*v) {
                    found = true;
                    break;
                }
            }

            if !found {
                invalid.push(*v);
            }
        }

        if invalid.is_empty() {
            ticket.is_valid = true;
            return None;
        }

        Some(invalid)
    }

    pub fn determine_rule_order(&mut self, tickets: &[Ticket]) -> Result<()> {
        let mut acc = Vec::new();
        let mut used: HashSet<&Rule> = HashSet::new();

        if self.recur(0, &self.make_col_map(tickets), tickets, &mut used, &mut acc) {
            acc.reverse();
            self.rules = acc;
            return Ok(());
        }
        Err(AocError::NoValidRuleOrder)
    }

    pub fn make_col_map(&self, tickets: &[Ticket]) -> Vec<HashSet<&Rule>> {
        let mut rule_map = vec![HashSet::new(); self.rules.len()];

        for i in 0..self.rules.len() {
            if let Some(r) = rule_map.get_mut(i) {
                for rule in &self.rules {
                    let mut satisfied = true;

                    for ticket in tickets.iter() {
                        if !rule.includes(ticket.values[i]) {
                            satisfied = false;
                            break;
                        }
                    }

                    if satisfied {
                        r.insert(rule);
                    }
                }
            }
        }

        rule_map
    }

    pub fn determine_rule_order_fast(&mut self, tickets: &[Ticket]) -> Result<()> {
        let mut col_map = self.make_col_map(tickets);
        let mut indicies = (0..col_map.len()).collect::<HashSet<usize>>();

        while !indicies.is_empty() {
            for i in indicies.iter().cloned() {
                if col_map[i].len() == 1 {
                    if let Some(val) = col_map[i].iter().cloned().next() {
                        TicketValidator::prune(&mut col_map, i, val);
                        indicies.remove(&i);
                        break;
                    }
                }
            }
        }

        self.rules = col_map
            .into_iter()
            .map(|col| col.into_iter().next().unwrap().to_owned())
            .collect::<Vec<Rule>>();

        Ok(())
    }

    fn prune(col_map: &mut Vec<HashSet<&Rule>>, skip: usize, target: &Rule) {
        for i in 0..col_map.len() {
            if i == skip {
                continue;
            }

            if let Some(col) = col_map.get_mut(i) {
                col.remove(target);
            }
        }
    }

    pub fn recur<'a>(
        &self,
        index: usize,
        rule_map: &Vec<HashSet<&'a Rule>>,
        tickets: &[Ticket],
        used: &mut HashSet<&'a Rule>,
        acc: &mut Vec<Rule>,
    ) -> bool {
        if index >= rule_map.len() {
            return true;
        }

        if let Some(rules) = rule_map.get(index) {
            for rule in rules.iter() {
                if used.contains(*rule) {
                    continue;
                }

                let mut satisfied = true;
                for ticket in tickets {
                    if !rule.includes(ticket.values[index]) {
                        satisfied = false;
                        break;
                    }
                }

                if satisfied {
                    used.insert(rule);

                    if self.recur(index + 1, rule_map, tickets, used, acc) {
                        acc.push(rule.clone().to_owned());
                        return true;
                    }

                    used.remove(rule);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod range {
        use super::*;

        #[test]
        fn from_str() {
            assert_eq!(Range::from_str("1-3").unwrap(), Range(1, 3));
            assert_eq!(Range::from_str("123-244").unwrap(), Range(123, 244));
            assert!(Range::from_str("a-3").is_err());
            assert!(Range::from_str("1-b").is_err());
            assert!(Range::from_str("3").is_err());
            assert!(Range::from_str("1-3-4").is_err());
            assert!(Range::from_str("5-2").is_err());
        }

        #[test]
        fn includes() {
            let r = Range::from_str("1-3").unwrap();
            assert!(r.includes(1));
            assert!(r.includes(2));
            assert!(r.includes(3));

            assert!(!r.includes(0));
            assert!(!r.includes(4));
            assert!(!r.includes(5));
        }
    }

    mod rule {
        use super::*;

        #[test]
        fn from_str() {
            assert_eq!(
                Rule::from_str("class: 1-3 or 5-7").unwrap(),
                Rule::new("class", &vec![Range(1, 3), Range(5, 7)])
            );

            assert_eq!(
                Rule::from_str("foo: 3-5 or 50-77").unwrap(),
                Rule::new("foo", &vec![Range(3, 5), Range(50, 77)])
            );

            assert_eq!(
                Rule::from_str("bar: 3-5 or 50-77 or 10-12").unwrap(),
                Rule::new("bar", &vec![Range(3, 5), Range(50, 77), Range(10, 12)])
            );

            assert!(Rule::from_str("foo: 3- or 50-77").is_err());
            assert!(Rule::from_str("3- or 50-77").is_err());
        }

        #[test]
        fn inclues() {
            let r = Rule::from_str("class: 1-3 or 5-7").unwrap();

            assert!(r.includes(1));
            assert!(r.includes(2));
            assert!(r.includes(3));
            assert!(r.includes(5));
            assert!(r.includes(6));
            assert!(r.includes(7));

            assert!(!r.includes(0));
            assert!(!r.includes(4));
            assert!(!r.includes(8));
        }
    }

    mod ticket {
        use super::*;

        #[test]
        fn from_str() {
            assert_eq!(
                Ticket::from_str("7,1,14").unwrap(),
                Ticket::new(&vec![7, 1, 14])
            );

            assert!(Ticket::from_str("7,1,14,a").is_err());
        }
    }

    mod ticket_validator {
        use super::*;
        use crate::util::test_input;

        #[test]
        fn from_input() {
            let input = test_input(
                "
                class: 1-3 or 5-7
                row: 6-11 or 33-44
                seat: 13-40 or 45-50",
            );

            let t = TicketValidator::from_input(&input).unwrap();

            let expected = TicketValidator::new(&vec![
                Rule::new("class", &vec![Range(1, 3), Range(5, 7)]),
                Rule::new("row", &vec![Range(6, 11), Range(33, 44)]),
                Rule::new("seat", &vec![Range(13, 40), Range(45, 50)]),
            ]);

            assert_eq!(t, expected);
        }

        #[test]
        fn validate() {
            let input = test_input(
                "
                class: 1-3 or 5-7
                row: 6-11 or 33-44
                seat: 13-40 or 45-50",
            );

            let validator = TicketValidator::from_input(&input).unwrap();

            let mut ticket = Ticket::new(&vec![7, 3, 47]);
            let res = validator.validate(&mut ticket);
            assert!(ticket.is_valid);
            assert_eq!(res, None);

            let mut ticket = Ticket::new(&vec![40, 4, 50]);
            let res = validator.validate(&mut ticket);
            assert!(!ticket.is_valid);
            assert_eq!(res, Some(vec![4]));

            let mut ticket = Ticket::new(&vec![55, 2, 20]);
            let res = validator.validate(&mut ticket);
            assert!(!ticket.is_valid);
            assert_eq!(res, Some(vec![55]));
        }

        #[test]
        fn determine_rule_order() {
            let input = test_input(
                "
                    class: 0-1 or 4-19
                    row: 0-5 or 8-19
                    seat: 0-13 or 16-19
                ",
            );

            let mut validator = TicketValidator::from_input(&input).unwrap();

            let tickets = vec![
                Ticket::new(&vec![3, 9, 18]),
                Ticket::new(&vec![15, 1, 5]),
                Ticket::new(&vec![5, 14, 9]),
            ];

            validator.determine_rule_order(&tickets).unwrap();

            assert_eq!(
                validator.rules,
                vec![
                    Rule::from_str("row: 0-5 or 8-19").unwrap(),
                    Rule::from_str("class: 0-1 or 4-19").unwrap(),
                    Rule::from_str("seat: 0-13 or 16-19").unwrap(),
                ]
            )
        }

        #[test]
        fn determine_rule_order_fast() {
            let input = test_input(
                "
                    class: 0-1 or 4-19
                    row: 0-5 or 8-19
                    seat: 0-13 or 16-19
                ",
            );

            let mut validator = TicketValidator::from_input(&input).unwrap();

            let tickets = vec![
                Ticket::new(&vec![3, 9, 18]),
                Ticket::new(&vec![15, 1, 5]),
                Ticket::new(&vec![5, 14, 9]),
            ];

            validator.determine_rule_order_fast(&tickets).unwrap();

            assert_eq!(
                validator.rules,
                vec![
                    Rule::from_str("row: 0-5 or 8-19").unwrap(),
                    Rule::from_str("class: 0-1 or 4-19").unwrap(),
                    Rule::from_str("seat: 0-13 or 16-19").unwrap(),
                ]
            )
        }
    }
}
