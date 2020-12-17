use std::clone::Clone;
use std::cmp::Eq;
use std::hash::Hash;

use std::collections::{HashMap, HashSet};

pub trait Addressable {
    type Item;

    fn from_coord(x: i64, y: i64) -> Self::Item;
    fn neighbors(&self) -> Vec<Self::Item>;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate(i64, i64, i64);

impl Addressable for Coordinate {
    type Item = Coordinate;

    fn from_coord(x: i64, y: i64) -> Self::Item {
        Coordinate(x, y, 0)
    }

    fn neighbors(&self) -> Vec<Self::Item> {
        let mut neighbors = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }

                    neighbors.push(Coordinate(self.0 + i, self.1 + j, self.2 + k));
                }
            }
        }

        neighbors
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FourDCoordinate(i64, i64, i64, i64);

impl Addressable for FourDCoordinate {
    type Item = FourDCoordinate;

    fn from_coord(x: i64, y: i64) -> Self::Item {
        FourDCoordinate(x, y, 0, 0)
    }

    fn neighbors(&self) -> Vec<Self::Item> {
        let mut neighbors = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for n in -1..=1 {
                        if i == 0 && j == 0 && k == 0 && n == 0 {
                            continue;
                        }

                        neighbors.push(FourDCoordinate(
                            self.0 + i,
                            self.1 + j,
                            self.2 + k,
                            self.3 + n,
                        ));
                    }
                }
            }
        }

        neighbors
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    Inactive,
    Active,
}

#[derive(Debug, Clone)]
pub struct Grid<T>
where
    T: Addressable + Eq + Hash + Clone + Addressable<Item = T>,
{
    coordinates: HashMap<T, State>,
}

impl<T> Grid<T>
where
    T: Addressable + Eq + Hash + Clone + Addressable<Item = T>,
{
    pub fn from_input(input: &[String]) -> Grid<T> {
        let mut g = Grid::new();

        input.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| match ch {
                '#' => g.insert(T::from_coord(x as i64, y as i64)),
                '.' => {}
                _ => {}
            })
        });

        g
    }

    pub fn new() -> Grid<T> {
        Grid {
            coordinates: HashMap::new(),
        }
    }

    pub fn insert(&mut self, cube: T) {
        *self.coordinates.entry(cube).or_insert(State::Active) = State::Active;
    }

    pub fn active(&self) -> usize {
        self.coordinates.len()
    }

    pub fn candidates(&self) -> HashMap<T, usize> {
        let mut candidates: HashMap<T, usize> = HashMap::new();

        self.coordinates.keys().for_each(|c| {
            candidates.entry(c.to_owned()).or_insert(0);
            c.neighbors()
                .iter()
                .for_each(|n| *candidates.entry(n.to_owned()).or_insert(0) += 1);
        });

        candidates
    }

    pub fn boot(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.cycle();
        }
    }

    pub fn cycle(&mut self) {
        let mut next_coordinates = self.coordinates.clone();
        for (candidate, count) in self.candidates() {
            match self.coordinates.get(&candidate) {
                Some(State::Active) => {
                    if count < 2 || count > 3 {
                        // *next_coordinates.entry(candidate).or_insert(State::Inactive) = State::Inactive;
                        next_coordinates.remove(&candidate);
                    }
                }
                Some(State::Inactive) | None => {
                    if count == 3 {
                        *next_coordinates.entry(candidate).or_insert(State::Active) = State::Active;
                    }
                }
            }
        }

        self.coordinates = next_coordinates;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod coordinate {
        use super::*;
        #[test]
        fn neighbors() {
            let root = Coordinate(0, 0, 0);

            let mut expected = Vec::new();

            for i in -1..=1 {
                for j in -1..=1 {
                    for k in -1..=1 {
                        if i == 0 && j == 0 && k == 0 {
                            continue;
                        }

                        expected.push(Coordinate(root.0 + i, root.1 + j, root.2 + k));
                    }
                }
            }

            assert_eq!(root.neighbors(), expected);
            assert_eq!(expected.len(), 26);
        }
    }

    mod grid {
        use super::*;
        use crate::util::test_input;

        #[test]
        fn candidates() {
            let input = test_input(
                "
                .#.
                ..#
                ### ",
            );

            let mut g: Grid<Coordinate> = Grid::from_input(&input);

            g.cycle();

            assert_eq!(g.active(), 11);
        }

        #[test]
        fn boot() {
            let input = test_input(
                "
                .#.
                ..#
                ### ",
            );

            let mut g: Grid<Coordinate> = Grid::from_input(&input);

            g.boot(6);

            assert_eq!(g.active(), 112);

            let mut g: Grid<FourDCoordinate> = Grid::from_input(&input);

            g.boot(6);

            assert_eq!(g.active(), 848);
        }
    }
}
