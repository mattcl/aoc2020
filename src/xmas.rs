use crate::error::{AocError, Result};
use itertools::Itertools;

pub struct Document {
    data: Vec<u64>,
}

impl Document {
    pub fn new(input: &[String]) -> std::result::Result<Self, std::num::ParseIntError> {
        // TODO: implement fromiterator for our custom error enum - MCL - 2020-12-08
        Ok(Document {
            data: input
                .iter()
                .map(|line| line.parse::<u64>())
                .collect::<std::result::Result<Vec<u64>, std::num::ParseIntError>>()?,
        })
    }

    pub fn find_outlier(&self, preamble_size: usize) -> Result<u64> {
        for (index, target) in self.data[preamble_size..self.data.len()].iter().enumerate() {
            if !self.check(index + preamble_size, preamble_size, target) {
                return Ok(*target);
            }
        }
        Err(AocError::NoOutlier)
    }

    fn check(&self, index: usize, preamble_size: usize, target: &u64) -> bool {
        let start = index - preamble_size;
        for pair in self.data[start..index].iter().combinations(2) {
            if pair.into_iter().sum::<u64>() == *target {
                return true;
            }
        }

        false
    }

    pub fn find_weakness_slow(&self, target: u64) -> Result<u64> {
        for i in 0..self.data.len() - 2 {
            for j in i + 2..self.data.len() {
                let sum = self.data[i..(j + 1)].iter().sum::<u64>();
                if sum == target {
                    let mut max = 0;
                    let mut min = 0;
                    for k in self.data[i..(j + 1)].iter() {
                        if *k < min || (min == max && max == 0) {
                            min = *k;
                        }

                        if *k > max {
                            max = *k;
                        }
                    }
                    return Ok(max + min);
                }

                if sum > target {
                    break;
                }
            }
        }

        Err(AocError::NoWeakness)
    }

    pub fn find_weakness(&self, target: u64) -> Result<u64> {
        let mut left = 0;
        let mut right = 1;

        if self.data.len() < 3 {
            return Err(AocError::NoWeakness);
        }

        let mut sum = self.data[left..(right + 1)].iter().sum::<u64>();

        loop {
            if sum == target {
                let mut max = 0;
                let mut min = 0;
                for i in self.data[left..(right + 1)].iter() {
                    if *i < min || (min == max && max == 0) {
                        min = *i;
                    }

                    if *i > max {
                        max = *i;
                    }
                }
                return Ok(max + min);
            } else if sum > target {
                sum -= self.data[left];
                left += 1;
            } else {
                right += 1;

                if right > self.data.len() {
                    break;
                }

                sum += self.data[right];
            }

            if left == right || right >= self.data.len() {
                break;
            }
        }

        Err(AocError::NoWeakness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "35".to_string(),
            "20".to_string(),
            "15".to_string(),
            "25".to_string(),
            "47".to_string(),
            "40".to_string(),
            "62".to_string(),
            "55".to_string(),
            "65".to_string(),
            "95".to_string(),
            "102".to_string(),
            "117".to_string(),
            "150".to_string(),
            "182".to_string(),
            "127".to_string(),
            "219".to_string(),
            "299".to_string(),
            "277".to_string(),
            "309".to_string(),
            "576".to_string(),
        ]
    }

    #[test]
    fn find_outlier() {
        let d = Document::new(&input()).unwrap();
        assert_eq!(d.find_outlier(5).unwrap(), 127);

        let d = Document::new(&input()[0..14]).unwrap();
        assert!(d.find_outlier(5).is_err());
    }

    #[test]
    fn find_weakness() {
        let d = Document::new(&input()).unwrap();
        assert_eq!(d.find_weakness_slow(127).unwrap(), 62);
        assert_eq!(d.find_weakness(127).unwrap(), 62);
    }
}
