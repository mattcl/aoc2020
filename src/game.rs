use crate::error::{AocError, Result};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct History {
    calls: usize,
    prev: usize,
    last: usize,
}

impl History {
    pub fn new() -> Self {
        History {
            calls: 0,
            prev: 0,
            last: 0,
        }
    }

    pub fn record(&mut self, turn: usize) {
        self.prev = self.last;
        self.last = turn;
        self.calls += 1;
    }

    pub fn was_first_time(&self) -> Result<bool> {
        if self.calls < 1 {
            return Err(AocError::GameError("History is empty".to_string()));
        }
        Ok(self.calls == 1)
    }

    pub fn delta(&self) -> Result<usize> {
        if self.calls < 2 {
            return Err(AocError::GameError(format!(
                "History too short: {:?}",
                self
            )));
        }
        Ok(self.last - self.prev)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    numbers: HashMap<usize, History>,
    last_spoken: usize,
    turn: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            numbers: HashMap::new(),
            last_spoken: 0,
            turn: 1,
        }
    }

    pub fn get_turn(&self) -> usize {
        self.turn
    }

    pub fn get_history(&self, number: &usize) -> Option<&History> {
        self.numbers.get(number)
    }

    pub fn get_last_spoken(&self) -> usize {
        self.last_spoken
    }

    pub fn initialize(&mut self, starting_numbers: &[usize]) {
        for i in starting_numbers {
            self.speak(*i);
        }
    }

    pub fn speak(&mut self, number: usize) {
        self.numbers
            .entry(number)
            .or_insert(History::new())
            .record(self.turn);
        self.turn += 1;
        self.last_spoken = number;
    }

    pub fn take_turn(&mut self) -> Result<usize> {
        if let Some(hist) = self.get_history(&self.last_spoken) {
            let to_speak = match hist.was_first_time()? {
                true => 0,
                false => hist.delta()?,
            };
            self.speak(to_speak);
            return Ok(to_speak);
        }

        Err(AocError::GameError(
            "Game has not been initialized".to_string(),
        ))
    }
}

impl FromStr for Game {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        let mut game = Game::new();

        game.initialize(
            &s.split(',')
                .map(|part| part.parse::<usize>())
                .collect::<std::result::Result<Vec<usize>, std::num::ParseIntError>>()?,
        );

        Ok(game)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game, total numbers: {}", self.numbers.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod history {
        use super::*;

        #[test]
        fn was_first_time() {
            let h = History::new();
            assert!(h.was_first_time().is_err());

            let h = History {
                prev: 0,
                last: 3232,
                calls: 1,
            };
            assert_eq!(h.was_first_time().unwrap(), true);

            let h = History {
                prev: 2,
                last: 3232,
                calls: 2,
            };
            assert_eq!(h.was_first_time().unwrap(), false);
        }

        #[test]
        fn delta() {
            let h = History {
                prev: 0,
                last: 1,
                calls: 1,
            };
            assert!(h.delta().is_err());

            let h = History {
                prev: 2,
                last: 5,
                calls: 2,
            };
            assert_eq!(h.delta().unwrap(), 3);

            let h = History {
                prev: 5,
                last: 9,
                calls: 4,
            };
            assert_eq!(h.delta().unwrap(), 4);
        }
    }

    mod game {
        use super::*;

        #[test]
        fn from_str() {
            let game = Game::from_str("0,3,6").unwrap();

            let mut expected = Game::new();
            expected.initialize(&[0, 3, 6]);

            assert_eq!(game, expected);
        }

        #[test]
        fn initialize() {
            let mut game = Game::new();
            game.initialize(&[0, 3, 6]);

            assert_eq!(
                game.get_history(&0),
                Some(&History {
                    prev: 0,
                    last: 1,
                    calls: 1
                })
            );
            assert_eq!(
                game.get_history(&3),
                Some(&History {
                    prev: 0,
                    last: 2,
                    calls: 1
                })
            );
            assert_eq!(
                game.get_history(&6),
                Some(&History {
                    prev: 0,
                    last: 3,
                    calls: 1
                })
            );
            assert_eq!(game.get_turn(), 4);
        }

        #[test]
        fn speak() {
            let mut game = Game::new();

            game.speak(1);
            assert_eq!(
                game.get_history(&1),
                Some(&History {
                    prev: 0,
                    last: 1,
                    calls: 1
                })
            );

            game.speak(1);
            assert_eq!(
                game.get_history(&1),
                Some(&History {
                    prev: 1,
                    last: 2,
                    calls: 2
                })
            );
        }

        #[test]
        fn take_turn() {
            let mut game = Game::new();
            game.initialize(&[0, 3, 6]);

            assert_eq!(game.take_turn().unwrap(), 0);
            assert_eq!(game.take_turn().unwrap(), 3);
            assert_eq!(game.take_turn().unwrap(), 3);
        }

        #[test]
        fn example_input() {
            let mut game = Game::new();
            game.initialize(&[0, 3, 6]);

            while game.get_turn() <= 2020 {
                game.take_turn().unwrap();
            }

            assert_eq!(game.get_last_spoken(), 436);
        }
    }
}
