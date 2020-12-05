use crate::error::{AocError, Result};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    pub fn new(row: usize, col: usize) -> Self {
        Seat { row: row, col: col }
    }

    pub fn from_id(id: usize) -> Self {
        Seat::new(id / 8, id % 8)
    }

    pub fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

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
                _ => {
                    return Err(AocError::SeatNotFound(
                        format!("invalid character {}", ch).to_string(),
                    ))
                }
            }

            if start == end {
                return Ok(start);
            }
        }

        Err(AocError::SeatNotFound(self.locator.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
