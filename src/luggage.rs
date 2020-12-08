use crate::error::{AocError, Result};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Rule {
    pub bag_name: String,
    contents: HashMap<String, usize>,
}

impl Rule {
    pub fn from_input(input: &[String]) -> Result<Vec<Self>> {
        input
            .iter()
            .map(|spec| Rule::new(spec))
            .collect::<Result<Vec<Self>>>()
    }

    pub fn new(spec: &str) -> Result<Self> {
        let res: Vec<&str> = spec.split(" bags contain ").collect();

        if res.len() != 2 {
            return Err(AocError::InvalidRule(spec.to_string()));
        }

        let contents: HashMap<String, usize> = res[1]
            .split(',')
            .map(|quantity| Rule::parse_quantity(quantity.trim()))
            .collect::<Result<Vec<Option<(String, usize)>>>>()?
            .into_iter()
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect();

        Ok(Rule {
            bag_name: res[0].to_string(),
            contents: contents,
        })
    }

    fn parse_quantity(quantities: &str) -> Result<Option<(String, usize)>> {
        if quantities == "no other bags." {
            return Ok(None);
        }

        let res: Vec<&str> = quantities.split(' ').collect();

        if res.len() != 4 {
            return Err(AocError::InvalidRule(quantities.to_string()));
        }

        let count = res[0].parse::<usize>()?;

        let name = format!("{} {}", res[1], res[2]);

        Ok(Some((name, count)))
    }

    pub fn contains_bag(&self, target: &str) -> bool {
        self.contents.contains_key(target)
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn num_immediate_bags(&self) -> usize {
        self.contents.values().sum()
    }
}

pub struct Ruleset {
    rules: HashMap<String, Rule>,
}

impl Ruleset {
    pub fn from_input(input: &[String]) -> Result<Self> {
        let rules: HashMap<String, Rule> = Rule::from_input(input)?
            .into_iter()
            .map(|rule| (rule.bag_name.clone(), rule.clone()))
            .collect();

        Ok(Ruleset { rules })
    }

    pub fn target_in_bag(
        &self,
        target: &str,
        rule: &Rule,
        seen: &mut HashMap<String, bool>,
    ) -> bool {
        if rule.contains_bag(target) {
            return true;
        }

        for (n, _) in &rule.contents {
            if seen.contains_key(n) {
                continue;
            }

            seen.insert(n.to_string(), false);

            if let Some(rule) = self.rules.get(n) {
                if self.target_in_bag(target, rule, seen) {
                    return true;
                }
            } else {
                return false;
            }
        }

        return false;
    }

    pub fn get_num_possible_bags(&self, target: &str) -> Result<usize> {
        if !self.rules.contains_key(target) {
            return Err(AocError::UnknownBag(target.to_string()));
        }

        let mut count = 0;

        for (_, rule) in &self.rules {
            let mut seen = HashMap::new();
            if self.target_in_bag(target, rule, &mut seen) {
                count += 1;
            }
        }

        Ok(count)
    }

    pub fn get_num_possible_bags_parallel(&self, target: &str) -> Result<usize> {
        if !self.rules.contains_key(target) {
            return Err(AocError::UnknownBag(target.to_string()));
        }

        Ok(self
            .rules
            .par_iter()
            .map(|(_, rule)| {
                let mut seen = HashMap::new();
                self.target_in_bag(target, rule, &mut seen)
            })
            .filter(|res| *res)
            .count())
    }

    pub fn target_in_bag_memoized(
        &self,
        target: &str,
        rule: &Rule,
        seen: &mut HashMap<String, bool>,
    ) -> bool {
        if rule.contains_bag(target) {
            return true;
        }

        for (n, _) in &rule.contents {
            if let Some(did_see) = seen.get(n) {
                if *did_see {
                    return true;
                }
                continue;
            }

            if let Some(rule) = self.rules.get(n) {
                if self.target_in_bag_memoized(target, rule, seen) {
                    seen.insert(n.to_string(), true);
                    return true;
                } else {
                    seen.insert(n.to_string(), false);
                }
            } else {
                return false;
            }
        }

        return false;
    }

    pub fn get_num_possible_bags_memoized(&self, target: &str) -> Result<usize> {
        if !self.rules.contains_key(target) {
            return Err(AocError::UnknownBag(target.to_string()));
        }

        let mut count = 0;
        let mut seen_cache: HashMap<String, bool> = HashMap::new();

        for (name, rule) in &self.rules {
            if let Some(did_see) = seen_cache.get(name) {
                if *did_see {
                    count += 1;
                }
            } else {
                if self.target_in_bag_memoized(target, rule, &mut seen_cache) {
                    seen_cache.insert(name.clone(), true);
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    pub fn count_r(&self, rule: &Rule) -> usize {
        if rule.is_empty() {
            return 0;
        }

        let mut count = rule.num_immediate_bags();

        for (name, n) in &rule.contents {
            if let Some(rule) = self.rules.get(name) {
                count += n * self.count_r(rule);
            }
        }

        return count;
    }

    pub fn count_bags(&self, target: &str) -> Result<usize> {
        if let Some(rule) = self.rules.get(target) {
            Ok(self.count_r(rule))
        } else {
            Err(AocError::UnknownBag(target.to_string()))
        }
    }

    pub fn count_r_memoized(&self, rule: &Rule, seen: &mut HashMap<String, usize>) -> usize {
        if rule.is_empty() {
            return 0;
        }

        let mut count = rule.num_immediate_bags();

        for (name, n) in &rule.contents {
            if let Some(cached_count) = seen.get(name) {
                count += *cached_count * n;
            } else {
                if let Some(rule) = self.rules.get(name) {
                    let val = self.count_r_memoized(rule, seen);
                    count += n * val;
                }
            }
        }

        seen.insert(rule.bag_name.clone(), count);
        return count;
    }

    pub fn count_bags_memoized(&self, target: &str) -> Result<usize> {
        if let Some(rule) = self.rules.get(target) {
            let mut seen = HashMap::with_capacity(self.rules.len());
            Ok(self.count_r_memoized(rule, &mut seen))
        } else {
            Err(AocError::UnknownBag(target.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ]
    }

    mod rule {
        use super::*;

        fn obj() -> Rule {
            Rule::new("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.")
                .expect("could not create rule")
        }

        #[test]
        fn from_input() {
            let res = Rule::from_input(&input()).expect("could not load all input");
            assert_eq!(res.len(), 9);
        }

        #[test]
        fn quantity_parsing() {
            let res = Rule::parse_quantity("a bright white bag");
            assert!(res.is_err());

            let res = Rule::parse_quantity("1 white bag");
            assert!(res.is_err());

            let res = Rule::parse_quantity("1 white bags foo bar");
            assert!(res.is_err());

            let res = Rule::parse_quantity("1 bright white bag")
                .expect("Unexpected failure to parse quantity");
            assert_eq!(res, Some(("bright white".to_string(), 1)));

            let res = Rule::parse_quantity("2 muted yellow bags.")
                .expect("Unexpected failure to parse quantity");
            assert_eq!(res, Some(("muted yellow".to_string(), 2)));

            let res = Rule::parse_quantity("no other bags.")
                .expect("Unexpected failure to parse quantity");
            assert_eq!(res, None);
        }

        #[test]
        fn contains_bag() {
            let rule = obj();

            assert!(rule.contains_bag("faded blue"));
            assert!(rule.contains_bag("dotted black"));
            assert!(!rule.contains_bag("foo black"));

            let rule =
                Rule::new("faded blue bags contain no other bags.").expect("could not create rule");
            assert!(!rule.contains_bag("plum red"));
        }

        #[test]
        fn is_empty() {
            assert!(!obj().is_empty());

            let rule =
                Rule::new("faded blue bags contain no other bags.").expect("could not create rule");
            assert!(rule.is_empty());
        }

        #[test]
        fn num_immediate_bags() {
            assert_eq!(obj().num_immediate_bags(), 11);

            let rule =
                Rule::new("faded blue bags contain no other bags.").expect("could not create rule");
            assert_eq!(rule.num_immediate_bags(), 0);

            let rule = Rule::new("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.")
                .expect("could not create rule");
            assert_eq!(rule.num_immediate_bags(), 3);
        }
    }

    mod ruleset {
        use super::*;

        #[test]
        fn from_input() {
            let res = Ruleset::from_input(&input()).expect("could not load ruleset");
            assert_eq!(res.rules.len(), 9);
        }

        #[test]
        fn get_num_possible_bags() {
            let rules = Ruleset::from_input(&input()).expect("could not load ruleset");
            let res = rules
                .get_num_possible_bags("shiny gold")
                .expect("could not find possible bags");
            assert_eq!(res, 4);

            let res = rules
                .get_num_possible_bags_parallel("shiny gold")
                .expect("could not find possible bags");
            assert_eq!(res, 4);

            let res = rules
                .get_num_possible_bags_memoized("shiny gold")
                .expect("could not find possible bags");
            assert_eq!(res, 4);
        }

        #[test]
        fn count_bags() {
            let rules = Ruleset::from_input(&input()).expect("could not load ruleset");
            let res = rules
                .count_bags("shiny gold")
                .expect("could not count bags");
            assert_eq!(res, 32);

            let res = rules
                .count_bags_memoized("shiny gold")
                .expect("could not count bags");
            assert_eq!(res, 32);
        }
    }
}
