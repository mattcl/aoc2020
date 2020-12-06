use crate::error::{AocError, Result};
use rayon::prelude::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    pub fn new(row: usize, col: usize) -> Self {
        Seat { row, col }
    }

    pub fn from_id(id: usize) -> Self {
        Seat::new(id / 8, id % 8)
    }

    pub fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Pass {
    locator: String,
    row_locator: String,
    col_locator: String,
}

impl Pass {
    pub fn new(locator: &str) -> Result<Self> {
        if locator.len() != 10 {
            return Err(AocError::InvalidLocator(locator.to_string()));
        }
        let (row_locator, col_locator) = locator.split_at(7);

        Ok(Pass {
            locator: locator.to_string(),
            row_locator: row_locator.to_string(),
            col_locator: col_locator.to_string(),
        })
    }

    pub fn seat(&self) -> Result<Seat> {
        Ok(Seat {
            row: self.bsearch(&self.row_locator, 127)?,
            col: self.bsearch(&self.col_locator, 7)?,
        })
    }

    fn bsearch(&self, locator: &str, num: usize) -> Result<usize> {
        let converted = locator.chars().map(|ch| match ch {
            'L' => 'F',
            'R' => 'B',
            _ => ch,
        });

        let mut start = 0;
        let mut end = num;

        for ch in converted {
            let mid = end - ((end - start) / 2);

            match ch {
                'F' => end = mid - 1,
                'B' => start = mid,
                _ => return Err(AocError::SeatNotFound(format!("invalid character {}", ch))),
            }

            if start == end {
                return Ok(start);
            }
        }

        Err(AocError::SeatNotFound(self.locator.clone()))
    }
}

// the following four functions are just playing around with performance of
// result handling and whatnot
pub fn find_highest_id(locators: &[String]) -> Result<usize> {
    locators
        .iter()
        .map(|line| Pass::new(&line))
        .map(|pass| pass.map_or_else(Err, |p| p.seat()))
        .map(|seat| seat.map_or_else(Err, |s| Ok(s.id())))
        .collect::<Result<Vec<usize>>>()?
        .into_iter()
        .max()
        .ok_or_else(|| AocError::InvalidLocator("No locators".to_string()))
}

pub fn find_highest_id_bad_errors(locators: &[String]) -> Result<usize> {
    locators
        .iter()
        .map(|line| Pass::new(&line).unwrap())
        .map(|pass| pass.seat().unwrap())
        .map(|seat| seat.id())
        .max()
        .ok_or_else(|| AocError::InvalidLocator("No locators".to_string()))
}

pub fn find_highest_id_par_bad_errors(locators: &[String]) -> Result<usize> {
    locators
        .par_iter()
        .map(|line| Pass::new(&line).unwrap())
        .map(|pass| pass.seat().unwrap())
        .map(|seat| seat.id())
        .max()
        .ok_or_else(|| AocError::InvalidLocator("No locators".to_string()))
}

pub fn find_highest_id_par(locators: &[String]) -> Result<usize> {
    locators
        .par_iter()
        .map(|line| Pass::new(&line))
        .map(|pass| pass.map_or_else(Err, |p| p.seat()))
        .map(|seat| seat.map_or_else(Err, |s| Ok(s.id())))
        .collect::<Result<Vec<usize>>>()?
        .into_par_iter()
        .max()
        .ok_or_else(|| AocError::InvalidLocator("No locators".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn findind_highest_id() {
        let locators = vec![
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];

        assert_eq!(find_highest_id(&locators).expect("input error"), 820);
        assert_eq!(
            find_highest_id_bad_errors(&locators).expect("input error"),
            820
        );
        assert_eq!(
            find_highest_id_par_bad_errors(&locators).expect("input error"),
            820
        );
        assert_eq!(find_highest_id_par(&locators).expect("input error"), 820);
    }

    mod seat {
        use super::super::*;

        #[test]
        fn getting_id() {
            let s = Seat::new(44, 5);
            assert_eq!(s.id(), 357);
        }

        #[test]
        fn from_id() {
            let s = Seat::new(44, 5);
            assert_eq!(Seat::from_id(357), s);
        }
    }

    mod pass {
        use super::super::*;

        #[test]
        fn bsearch() {
            let p = Pass::new("BFFFBBFRRR").expect("Could not make Pass");

            assert_eq!(p.bsearch(&p.row_locator, 127).unwrap(), 70);
            assert_eq!(p.bsearch(&p.col_locator, 7).unwrap(), 7);

            let p = Pass::new("FFFBBBFRLL").expect("Could not make Pass");

            assert_eq!(p.bsearch(&p.row_locator, 127).unwrap(), 14);
            assert_eq!(p.bsearch(&p.col_locator, 7).unwrap(), 4);
        }

        #[test]
        fn finding_seat() {
            let p = Pass::new("BFFFBBFRRR").expect("Could not make Pass");

            let s = p.seat().expect("Could not find seat");
            assert_eq!(s.id(), 567);
        }
    }
}
