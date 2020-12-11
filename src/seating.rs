use crate::error::{AocError, Result};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Loc {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl Loc {
    pub fn from_char(c: char) -> Result<Loc> {
        match c {
            '#' => Ok(Loc::OccupiedSeat),
            'L' => Ok(Loc::EmptySeat),
            '.' => Ok(Loc::Floor),
            _ => Err(AocError::InvalidInput(format!(
                "Unknown location char {}",
                c
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Area {
    seats: Vec<Vec<Loc>>,
    search_range: Option<usize>,
    tipping_point: usize,
}

impl Area {
    pub fn new(
        input: &[String],
        search_range: Option<usize>,
        tipping_point: usize,
    ) -> Result<Self> {
        Ok(Area {
            seats: input
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| Loc::from_char(c))
                        .collect::<Result<Vec<Loc>>>()
                })
                .collect::<Result<Vec<Vec<Loc>>>>()?,
            search_range: search_range,
            tipping_point: tipping_point,
        })
    }

    fn search_range(&self) -> i64 {
        match self.search_range {
            Some(val) => val as i64,
            None => self.seats.len() as i64,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Loc> {
        if let Some(row) = self.seats.get(row) {
            if let Some(col) = row.get(col) {
                return Some(col.clone());
            }
        }
        None
    }

    pub fn occupied_neighbors(&self, row: usize, col: usize) -> Result<usize> {
        if let Some(_) = self.get(row, col) {
            let row = row as i64;
            let col = col as i64;

            let mut count = 0;

            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    for k in 1..=self.search_range() {
                        let new_row = row + (i * k);
                        let new_col = col + (j * k);

                        if new_row < 0 || new_col < 0 {
                            break;
                        }

                        if let Some(candidate) = self.get(new_row as usize, new_col as usize) {
                            match candidate {
                                Loc::OccupiedSeat => {
                                    count += 1;
                                    break;
                                }
                                Loc::EmptySeat => {
                                    break;
                                }
                                _ => {}
                            }
                        } else {
                            break;
                        }
                    }
                }
            }

            return Ok(count);
        }
        Err(AocError::SeatDoesNotExist((row, col)))
    }

    pub fn step(&self) -> Result<Self> {
        let mut new_seats = self.seats.clone();

        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {
                match self.get(row, col) {
                    Some(Loc::EmptySeat) | Some(Loc::OccupiedSeat) => {
                        let num_neighbors = self.occupied_neighbors(row, col)?;
                        if num_neighbors == 0 {
                            new_seats[row][col] = Loc::OccupiedSeat;
                        } else if num_neighbors >= self.tipping_point {
                            new_seats[row][col] = Loc::EmptySeat;
                        }
                    }
                    _ => {}
                };
            }
        }

        Ok(Area {
            seats: new_seats,
            search_range: self.search_range,
            tipping_point: self.tipping_point,
        })
    }

    pub fn occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .map(|row| row.iter().filter(|s| **s == Loc::OccupiedSeat).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod loc {
        use super::super::*;

        #[test]
        fn from_char() {
            assert_eq!(Loc::from_char('#').unwrap(), Loc::OccupiedSeat);
            assert_eq!(Loc::from_char('L').unwrap(), Loc::EmptySeat);
            assert_eq!(Loc::from_char('.').unwrap(), Loc::Floor);
            assert!(Loc::from_char('a').is_err());
            assert!(Loc::from_char('b').is_err());
        }
    }

    mod area {
        use super::super::*;
        use crate::util::test_input;

        fn input() -> Vec<String> {
            vec![
                "L.LL.LL.LL".to_string(),
                "LLLLLLL.LL".to_string(),
                "L.L.L..L..".to_string(),
                "LLLL.LL.LL".to_string(),
                "L.LL.LL.LL".to_string(),
                "L.LLLLL.LL".to_string(),
                "..L.L.....".to_string(),
                "LLLLLLLLLL".to_string(),
                "L.LLLLLL.L".to_string(),
                "L.LLLLL.LL".to_string(),
            ]
        }

        #[test]
        fn new() {
            assert!(Area::new(&input(), Some(1), 4).is_ok());
        }

        #[test]
        fn get() {
            let a = Area::new(&input(), Some(1), 4).unwrap();
            assert_eq!(a.get(0, 0), Some(Loc::EmptySeat));
            assert_eq!(a.get(2, 3), Some(Loc::Floor));
            assert_eq!(a.get(10, 0), None);
            assert_eq!(a.get(0, 25), None);
        }

        #[test]
        fn occupied_neighbors() {
            let input = vec![
                "L.LL.LL.LL".to_string(),
                "LLL#L#L.LL".to_string(),
                "#.L.##.L..".to_string(),
                "#LLL.LL.##".to_string(),
                "#.LL.LL.LL".to_string(),
                "L.LLLLL.#L".to_string(),
                "..L.L.....".to_string(),
                "LLL###LLLL".to_string(),
                "L.L#L#LL.L".to_string(),
                "L.L###L.LL".to_string(),
            ];

            let a = Area::new(&input, Some(1), 4).unwrap();

            assert_eq!(a.occupied_neighbors(3, 1).unwrap(), 3);
            assert_eq!(a.occupied_neighbors(8, 4).unwrap(), 8);

            let state = vec![
                ".##.##.".to_string(),
                "#.#.#.#".to_string(),
                "##...##".to_string(),
                "...L...".to_string(),
                "##...##".to_string(),
                "#.#.#.#".to_string(),
                ".##.##.".to_string(),
            ];

            let a = Area::new(&state, None, 4).unwrap();

            assert_eq!(a.occupied_neighbors(3, 3).unwrap(), 0);
            assert_eq!(a.occupied_neighbors(3, 1).unwrap(), 6);
        }

        #[test]
        fn step() {
            let state = vec![
                "#.#L.L#.##".to_string(),
                "#LLL#LL.L#".to_string(),
                "L.#.L..#..".to_string(),
                "#L##.##.L#".to_string(),
                "#.#L.LL.LL".to_string(),
                "#.#L#L#.##".to_string(),
                "..L.L.....".to_string(),
                "#L#L##L#L#".to_string(),
                "#.LLLLLL.L".to_string(),
                "#.#L#L#.##".to_string(),
            ];

            let a = Area::new(&state, Some(1), 4).unwrap();

            assert_eq!(a.step().unwrap(), a);

            let state = vec![
                "#.##.##.##".to_string(),
                "#######.##".to_string(),
                "#.#.#..#..".to_string(),
                "####.##.##".to_string(),
                "#.##.##.##".to_string(),
                "#.#####.##".to_string(),
                "..#.#.....".to_string(),
                "##########".to_string(),
                "#.######.#".to_string(),
                "#.#####.##".to_string(),
            ];

            let a = Area::new(&input(), Some(1), 4).unwrap();
            let b = Area::new(&state, Some(1), 4).unwrap();

            assert_eq!(a.step().unwrap(), b);

            let state = vec![
                "#.L#.L#.L#".to_string(),
                "#LLLLLL.LL".to_string(),
                "L.L.L..#..".to_string(),
                "##L#.#L.L#".to_string(),
                "L.L#.LL.L#".to_string(),
                "#.LLLL#.LL".to_string(),
                "..#.L.....".to_string(),
                "LLL###LLL#".to_string(),
                "#.LLLLL#.L".to_string(),
                "#.L#LL#.L#".to_string(),
            ];

            let a = Area::new(&state, None, 5).unwrap();
            assert_eq!(a.step().unwrap(), a);
        }

        #[test]
        fn occupied_seats() {
            let state = test_input("
                 #.#L.L#.##
                 #LLL#LL.L#
                 L.#.L..#..
                 #L##.##.L#
                 #.#L.LL.LL
                 #.#L#L#.##
                 ..L.L.....
                 #L#L##L#L#
                 #.LLLLLL.L
                 #.#L#L#.##"
            );

            let a = Area::new(&state, Some(1), 4).unwrap();
            assert_eq!(a.occupied_seats(), 37);
        }
    }
}
