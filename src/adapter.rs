use std::cmp::{Ord, Ordering};
use std::collections::HashMap;

use crate::error::{AocError, Result};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Adapter {
    rating: i32,
}

impl Adapter {
    pub fn from_input(input: &[String]) -> Result<Vec<Self>> {
        input
            .iter()
            .map(|i| Self::new(i))
            .collect::<Result<Vec<Self>>>()
    }

    pub fn from_rating(rating: i32) -> Result<Self> {
        if rating < 0 {
            return Err(AocError::InputError("Raings must be positive".to_string()));
        }

        Ok(Adapter { rating })
    }

    pub fn new(input: &str) -> Result<Self> {
        let rating = input.parse::<i32>()?;

        if rating < 0 {
            return Err(AocError::InputError("Raings must be positive".to_string()));
        }

        Ok(Adapter { rating })
    }

    pub fn rating(&self) -> i32 {
        self.rating
    }

    pub fn output(&self) -> i32 {
        self.rating() + 3
    }

    pub fn can_be_plugged_into(&self, other: &Self) -> bool {
        let diff = self.rating() - other.output();

        diff <= 0 && diff >= -3
    }

    pub fn diff(&self, other: &Self) -> i32 {
        let diff = self.rating() - other.rating();
        diff.abs()
    }
}

// enable sorting
impl Ord for Adapter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rating.cmp(&other.rating)
    }
}

pub fn compute_diffs_in_chain(adapters: &[Adapter]) -> Result<i32> {
    if adapters.is_empty() {
        return Err(AocError::InvalidInput("No adapters".to_string()));
    }

    let mut adapters = adapters.to_vec();
    adapters.sort();

    if adapters[0].rating() > 3 {
        return Err(AocError::InvalidInput(
            "No adapters small enough".to_string(),
        ));
    }

    let mut counts: HashMap<i32, i32> = HashMap::new();

    let mut prev = &Adapter::new("0")?;

    for adapter in adapters.iter() {
        if !adapter.can_be_plugged_into(prev) {
            return Err(AocError::NoAdapterChain);
        }
        let diff = adapter.diff(prev);
        *counts.entry(diff).or_insert(0) += 1;
        prev = adapter;
    }

    if let Some(one) = counts.get(&1) {
        if let Some(three) = counts.get(&3) {
            return Ok(*one * (*three + 1));
        }
    }

    Err(AocError::InvalidInput(
        "missing deltas of 1 or missing deltas of 3".to_string(),
    ))
}

pub fn permutations(adapters: &[Adapter]) -> Result<usize> {
    if adapters.is_empty() {
        return Err(AocError::InvalidInput("No adapters".to_string()));
    }

    let mut chain = vec![Adapter::from_rating(0)?];
    chain.extend(adapters.to_vec());
    chain.sort();
    chain.push(Adapter::from_rating(chain[chain.len() - 1].rating() + 3)?);

    let mut cache = HashMap::new();
    Ok(recur(&chain, 0, &mut cache))
}

fn recur(adapters: &[Adapter], start: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if start >= adapters.len() - 1 {
        return 1;
    }

    let mut local_count = 0;

    for i in (start + 1)..adapters.len() {
        if adapters[i].can_be_plugged_into(&adapters[start]) {
            if let Some(cached) = cache.get(&i) {
                local_count += *cached;
            } else {
                local_count += recur(adapters, i, cache);
            }
        } else {
            break;
        }
    }

    cache.insert(start, local_count);
    return local_count;
}

pub fn permutations_faster(adapters: &[Adapter]) -> Result<i64> {
    if adapters.is_empty() {
        return Err(AocError::InvalidInput("No adapters".to_string()));
    }

    let mut chain = vec![Adapter::from_rating(0)?];
    chain.extend(adapters.to_vec());
    chain.sort();
    chain.push(Adapter::from_rating(chain[chain.len() - 1].rating() + 3)?);

    let mut cache = vec![-1; chain.len()];
    Ok(recur_faster(&chain, 0, &mut cache))
}

fn recur_faster(adapters: &[Adapter], start: usize, cache: &mut [i64]) -> i64 {
    if start >= adapters.len() - 1 {
        return 1;
    }

    let mut local_count = 0;

    for i in (start + 1)..adapters.len() {
        if adapters[i].can_be_plugged_into(&adapters[start]) {
            if cache[i] > -1 {
                local_count += cache[i];
            } else {
                local_count += recur_faster(adapters, i, cache);
            }
        } else {
            break;
        }
    }

    cache[start] = local_count;
    return local_count;
}

pub fn permutations_slow(adapters: &[Adapter]) -> Result<usize> {
    if adapters.is_empty() {
        return Err(AocError::InvalidInput("No adapters".to_string()));
    }

    let mut chain = vec![Adapter::from_rating(0)?];
    chain.extend(adapters.to_vec());
    chain.sort();
    chain.push(Adapter::from_rating(chain[chain.len() - 1].rating() + 3)?);

    Ok(recur_slow(&chain, 0))
}

fn recur_slow(adapters: &[Adapter], start: usize) -> usize {
    if start >= adapters.len() - 1 {
        return 1;
    }

    let mut local_count = 0;

    for i in (start + 1)..adapters.len() {
        if adapters[i].can_be_plugged_into(&adapters[start]) {
            local_count += recur_slow(adapters, i);
        } else {
            break;
        }
    }

    return local_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "28".to_string(),
            "33".to_string(),
            "18".to_string(),
            "42".to_string(),
            "31".to_string(),
            "14".to_string(),
            "46".to_string(),
            "20".to_string(),
            "48".to_string(),
            "47".to_string(),
            "24".to_string(),
            "23".to_string(),
            "49".to_string(),
            "45".to_string(),
            "19".to_string(),
            "38".to_string(),
            "39".to_string(),
            "11".to_string(),
            "1".to_string(),
            "32".to_string(),
            "25".to_string(),
            "35".to_string(),
            "8".to_string(),
            "17".to_string(),
            "7".to_string(),
            "9".to_string(),
            "4".to_string(),
            "2".to_string(),
            "34".to_string(),
            "10".to_string(),
            "3".to_string(),
        ]
    }

    #[test]
    fn computing_diffs() {
        let adapters = Adapter::from_input(&input()).unwrap();
        let res = compute_diffs_in_chain(&adapters).expect("Could not find chain");

        assert_eq!(res, 220);
    }

    #[test]
    fn finding_permutations() {
        let adapters = Adapter::from_input(&input()).unwrap();
        let res = permutations(&adapters).expect("Could not count permutations");
        assert_eq!(res, 19208);

        let res = permutations_faster(&adapters).expect("Could not count permutations");
        assert_eq!(res, 19208);

        let res = permutations_slow(&adapters).expect("Could not count permutations");
        assert_eq!(res, 19208);
    }

    mod adapter {
        use super::*;

        #[test]
        fn from_input() {
            let res = Adapter::from_input(&input()).unwrap();
        }

        #[test]
        fn new() {
            let a = Adapter::new("123").unwrap();

            assert_eq!(a.rating(), 123);
            assert_eq!(a.output(), 126);

            assert!(Adapter::new("-123").is_err());
            assert!(Adapter::new("-").is_err());
            assert!(Adapter::new("foo").is_err());
        }

        #[test]
        fn compatible() {
            let a = Adapter::new("3").unwrap();
            let b = Adapter::new("6").unwrap();

            assert!(b.can_be_plugged_into(&a));
            assert!(!a.can_be_plugged_into(&b));

            let a = Adapter::new("2").unwrap();
            let b = Adapter::new("6").unwrap();

            assert!(!b.can_be_plugged_into(&a));

            let a = Adapter::new("2").unwrap();
            let b = Adapter::new("3").unwrap();

            assert!(b.can_be_plugged_into(&a));

            let a = Adapter::new("7").unwrap();
            let b = Adapter::new("3").unwrap();

            assert!(!b.can_be_plugged_into(&a));
        }

        #[test]
        fn diff() {
            let a = Adapter::new("7").unwrap();
            let b = Adapter::new("3").unwrap();

            assert_eq!(b.diff(&a), 4);
            assert_eq!(a.diff(&b), 4);
        }

        #[test]
        fn sortable() {
            let mut res = Adapter::from_input(&input()).unwrap();
            res.sort();
            assert_eq!(res[0].rating(), 1);
            assert_eq!(res[res.len() - 1].rating(), 49);
        }
    }
}
