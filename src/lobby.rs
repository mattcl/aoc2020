use crate::error::{AocError, Result};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Address(Vec<Dir>);

impl Address {
    pub fn from_input(input: &[String]) -> Result<Vec<Address>> {
        input.iter().map(|line| Dir::parse_instructions(line)).collect::<Result<Vec<Address>>>()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Dir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast
}

impl Dir {
    pub fn parse_instructions(s: &str) -> Result<Address> {
        let mut chars = s.chars().peekable();
        let mut res = Vec::new();

        loop {
            if let Some(first) = chars.next() {
                match first {
                    'e' => res.push(Dir::East),
                    'w' => res.push(Dir::West),
                    's' if Some(&'e') == chars.peek() => {
                        chars.next();
                        res.push(Dir::SouthEast);
                    },
                    's' if Some(&'w') == chars.peek() => {
                        chars.next();
                        res.push(Dir::SouthWest);
                    },
                    'n' if Some(&'e') == chars.peek() => {
                        chars.next();
                        res.push(Dir::NorthEast);
                    },
                    'n' if Some(&'w') == chars.peek() => {
                        chars.next();
                        res.push(Dir::NorthWest);
                    },
                    _ => return Err(AocError::InvalidInput("cannot parse instructions".to_string()))
                }
            } else {
                break;
            }
        }

        Ok(Address(res))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Face {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Coordinate(i64, i64);

impl Coordinate {
    pub fn shift(&self, dir: &Dir) -> Coordinate {
        match dir {
            Dir::East => Coordinate(self.0 + 2, self.1),
            Dir::West => Coordinate(self.0 - 2, self.1),
            Dir::NorthEast => Coordinate(self.0 + 1, self.1 + 1),
            Dir::NorthWest => Coordinate(self.0 - 1, self.1 + 1),
            Dir::SouthEast => Coordinate(self.0 + 1, self.1 - 1),
            Dir::SouthWest => Coordinate(self.0 - 1, self.1 - 1),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tile {
    pub coordinate: Coordinate,
    pub face: Face,
}

impl Tile {
    pub fn new(coordinate: Coordinate) -> Self {
        Tile {
            coordinate: coordinate,
            face: Face::White,
        }
    }

    pub fn flip(&mut self) {
        match self.face {
            Face::White => self.face = Face::Black,
            Face::Black => self.face = Face::White,
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Lobby {
    tiles: HashMap<Coordinate, Tile>,
}

impl Lobby {
    pub fn new() -> Self {
        let mut l = Lobby::default();
        l.tiles.insert(Coordinate(0, 0), Tile::new(Coordinate(0, 0)));
        l
    }

    pub fn flip(&mut self, address: &Address) {
        let mut dirs = address.0.iter();
        let mut cur = Coordinate(0, 0);

        loop {
            if let Some(dir) = dirs.next() {
                cur = cur.shift(dir);
            } else {
                break
            }
        }

        self.tiles.entry(cur).or_insert(Tile::new(cur)).flip();
    }

    pub fn count_tiles(&self, face: &Face) -> usize {
        self.tiles.values().filter(|t| t.face == *face).count()
    }

    pub fn candidates(&self) -> HashMap<Coordinate, usize> {
        let dirs = vec![Dir::East, Dir::West, Dir::SouthEast, Dir::SouthWest, Dir::NorthEast, Dir::NorthWest];
        let mut candidates: HashMap<Coordinate, usize> = HashMap::with_capacity(self.tiles.len() * 7);

        self.tiles.iter().for_each(|(c, t)| {
            candidates.entry(c.to_owned()).or_insert(0);

            dirs.iter().for_each(|dir| {
                let e = candidates.entry(c.shift(dir)).or_insert(0);
                if t.face == Face::Black {
                    *e += 1;
                }
            })
        });

        candidates
    }

    pub fn simulate(&mut self, generations: usize) {
        for _ in 0..generations {
            self.generation();
        }
    }

    pub fn generation(&mut self) {
        for (candidate, count) in self.candidates() {
            match self.tiles.get(&candidate) {
                Some(tile) if tile.face == Face::Black => {
                    if count == 0 || count > 2 {
                        self.tiles.remove(&candidate);
                    }
                }
                _ => {
                    if count == 2 {
                        self.tiles.entry(candidate).or_insert(Tile::new(candidate)).flip();
                    } else {
                        self.tiles.remove(&candidate);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_input;

    mod dir {
        use super::*;

        #[test]
        fn parse_instructions() {
            let res = Dir::parse_instructions("esenee").unwrap();
            let expected = Address(vec![Dir::East, Dir::SouthEast, Dir::NorthEast, Dir::East]);
            assert_eq!(res, expected);

            let res = Dir::parse_instructions("nwwswee").unwrap();
            let expected = Address(vec![Dir::NorthWest, Dir::West, Dir::SouthWest, Dir::East, Dir::East]);
            assert_eq!(res, expected);

            assert!(Dir::parse_instructions("nwwsween").is_err());
        }
    }

    mod coordinate {
        use super::*;

        #[test]
        fn shift() {
            //     (-1, 1) (1, 1)
            // (-2, 0) (0, 0) (2, 0)
            let c = Coordinate(0, 0);

            assert_eq!(c.shift(&Dir::East), Coordinate(2, 0));
            assert_eq!(c.shift(&Dir::West), Coordinate(-2, 0));
            assert_eq!(c.shift(&Dir::NorthEast), Coordinate(1, 1));
            assert_eq!(c.shift(&Dir::NorthWest), Coordinate(-1, 1));
        }
    }

    mod tile {
        use super::*;

        #[test]
        fn flip() {
            let mut t = Tile::new(Coordinate(2, 3));
            assert_eq!(t.face, Face::White);
            t.flip();
            assert_eq!(t.face, Face::Black);
            t.flip();
            assert_eq!(t.face, Face::White);
        }
    }

    mod lobby {
        use super::*;

        fn input() -> Vec<String> {
            test_input("
                sesenwnenenewseeswwswswwnenewsewsw
                neeenesenwnwwswnenewnwwsewnenwseswesw
                seswneswswsenwwnwse
                nwnwneseeswswnenewneswwnewseswneseene
                swweswneswnenwsewnwneneseenw
                eesenwseswswnenwswnwnwsewwnwsene
                sewnenenenesenwsewnenwwwse
                wenwwweseeeweswwwnwwe
                wsweesenenewnwwnwsenewsenwwsesesenwne
                neeswseenwwswnwswswnw
                nenwswwsewswnenenewsenwsenwnesesenew
                enewnwewneswsewnwswenweswnenwsenwsw
                sweneswneswneneenwnewenewwneswswnese
                swwesenesewenwneswnwwneseswwne
                enesenwswwswneneswsenwnewswseenwsese
                wnwnesenesenenwwnenwsewesewsesesew
                nenewswnwewswnenesenwnesewesw
                eneswnwswnwsenenwnwnwwseeswneewsenese
                neswnwewnwnwseenwseesewsenwsweewe
                wseweeenwnesenwwwswnew
            ")
        }

        #[test]
        fn flip() {
            let input = input();
            let addresses = Address::from_input(&input).unwrap();

            let mut lobby = Lobby::new();

            addresses.iter().for_each(|address| lobby.flip(address));

            assert_eq!(lobby.count_tiles(&Face::Black), 10);
        }

        #[test]
        fn generation() {
            let input = input();
            let addresses = Address::from_input(&input).unwrap();
            let mut lobby = Lobby::new();
            addresses.iter().for_each(|address| lobby.flip(address));
            //Day 1: 15
            //Day 2: 12
            //Day 3: 25
            //Day 4: 14
            //Day 5: 23
            //Day 6: 28
            //Day 7: 41
            //Day 8: 37
            //Day 9: 49
            //Day 10: 37
            //

            // sanity check
            assert_eq!(lobby.count_tiles(&Face::Black), 10);

            lobby.generation();
            assert_eq!(lobby.count_tiles(&Face::Black), 15);

            lobby.generation();
            assert_eq!(lobby.count_tiles(&Face::Black), 12);

            lobby.simulate(3);
            assert_eq!(lobby.count_tiles(&Face::Black), 23);
        }

        #[test]
        fn simulate() {
            let input = input();
            let addresses = Address::from_input(&input).unwrap();
            let mut lobby = Lobby::new();
            addresses.iter().for_each(|address| lobby.flip(address));
            lobby.simulate(100);
            assert_eq!(lobby.count_tiles(&Face::Black), 2208);
        }
    }
}
