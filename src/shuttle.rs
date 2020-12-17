use crate::error::{AocError, Result};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Bus {
    id: usize,
    in_service: bool,
}

impl Bus {
    pub fn new(id: usize) -> Self {
        Bus {
            id: id,
            in_service: id > 0,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn in_service(&self) -> bool {
        self.in_service
    }

    pub fn departs_at(&self, time: usize) -> bool {
        time % self.id() == 0
    }
}

impl fmt::Display for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.id())
    }
}

impl FromStr for Bus {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "x" => Ok(Bus {
                id: 0,
                in_service: false,
            }),
            _ => Ok(Bus {
                id: s.parse::<usize>()?,
                in_service: true,
            }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Schedule {
    buses: Vec<Bus>,
}

impl Schedule {
    pub fn earliest_departure(&self, start: usize) -> Option<(usize, Bus)> {
        let buses = self.buses_in_service().collect::<Vec<&Bus>>();

        for i in start..(start + 1_000_000) {
            for bus in &buses {
                if bus.departs_at(i) {
                    return Some((i - start, *bus.clone()));
                }
            }
        }

        None
    }

    pub fn buses_in_service(&self) -> impl Iterator<Item = &Bus> {
        self.buses.iter().filter(|bus| bus.in_service)
    }

    pub fn sync_departures(&self) -> Option<usize> {
        let mut max = 0;
        let mut max_offset = 0;

        for (offset, bus) in self.buses.iter().enumerate() {
            if bus.in_service() && bus.id() > max {
                max = bus.id();
                max_offset = offset;
            }
        }

        let mut second_max = 0;
        let mut second_max_offset = 0;

        for (offset, bus) in self.buses.iter().enumerate() {
            if bus.in_service() && bus.id() > second_max && bus.id() != max {
                second_max = bus.id();
                second_max_offset = offset;
            }
        }

        let first_intersection = self.calc_first_intersection(max_offset, second_max_offset);
        let mut g = Generator::new(max, second_max, first_intersection, max_offset);

        let buses = self
            .buses
            .iter()
            .enumerate()
            .filter(|(_, bus)| bus.in_service())
            .collect::<Vec<(usize, &Bus)>>();

        if max > 0 {
            loop {
                let next_candidate = g.next();
                if self.check_departures(next_candidate, &buses) {
                    return Some(next_candidate);
                }
            }
        }
        None
    }

    pub fn check_departures(&self, cur: usize, buses: &[(usize, &Bus)]) -> bool {
        for (offset, bus) in buses {
            if !bus.departs_at(cur + offset) {
                return false;
            }
        }
        true
    }

    fn calc_first_intersection(&self, bus_idx: usize, other_idx: usize) -> usize {
        let left = self.buses[bus_idx];
        let right = self.buses[other_idx];

        let mut cur = left.id() - bus_idx;
        loop {
            if (cur + other_idx) % right.id() == 0 {
                return cur;
            }
            cur += left.id();
        }
    }
}

impl FromStr for Schedule {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Schedule {
            buses: s
                .split(',')
                .map(|slice| Bus::from_str(slice))
                .collect::<Result<Vec<Bus>>>()?,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Generator {
    pub left: usize,
    pub right: usize,
    pub base: usize,
    pub n: usize,
    pub left_offset: usize,
}

impl Generator {
    pub fn new(left: usize, right: usize, first_intersection: usize, left_offset: usize) -> Self {
        let base = (first_intersection + left_offset) / left;
        Generator {
            left: left,
            right: right,
            base: base,
            n: 0,
            left_offset: left_offset,
        }
    }

    pub fn next(&mut self) -> usize {
        self.n += 1;
        self.current()
    }

    pub fn current(&self) -> usize {
        self.left * (self.base + (self.n * self.right)) - self.left_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bus {
        use super::*;

        #[test]
        fn from_str() {
            assert_eq!(
                Bus::from_str("x").unwrap(),
                Bus {
                    id: 0,
                    in_service: false
                }
            );
            assert_eq!(
                Bus::from_str("1").unwrap(),
                Bus {
                    id: 1,
                    in_service: true
                }
            );
            assert_eq!(
                Bus::from_str("111").unwrap(),
                Bus {
                    id: 111,
                    in_service: true
                }
            );
            assert_eq!(
                Bus::from_str("45").unwrap(),
                Bus {
                    id: 45,
                    in_service: true
                }
            );
            assert!(Bus::from_str("a").is_err());
        }
    }

    mod schedule {
        use super::*;

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct Slot(usize, Bus);

        impl fmt::Display for Slot {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} offset {}", self.1.id(), self.0)
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct Pair {
            left: Slot,
            right: Slot,
        }

        impl Pair {
            pub fn intersections(&self, n: usize) -> Vec<Intersect> {
                let mut vals = Vec::new();
                let mut cur = self.left.1.id() - self.left.0;
                let mut count = 0;
                loop {
                    if (cur + self.right.0) % self.right.1.id() == 0 {
                        count += 1;
                        vals.push(Intersect {
                            pair: self.clone(),
                            time: cur,
                        });
                    }

                    if count >= n {
                        break;
                    }

                    cur += self.left.1.id();
                }

                vals
            }
        }

        impl fmt::Display for Pair {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Pair ({}, {})", self.left, self.right)
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct Intersect {
            pair: Pair,
            time: usize,
        }

        impl Intersect {
            fn detail(&self, slot: &Slot) -> String {
                let factor = (self.time + slot.0) / slot.1.id();
                format!("({} * {} - {})", slot.1.id(), factor, slot.0)
            }
        }

        impl fmt::Display for Intersect {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{} - {} - left {} right {}",
                    self.pair,
                    self.time,
                    self.detail(&self.pair.left),
                    self.detail(&self.pair.right)
                )
            }
        }

        #[test]
        fn from_str() {
            let input = "7,13,x,x,59,x,31,19";

            let expected = Schedule {
                buses: vec![
                    Bus {
                        id: 7,
                        in_service: true,
                    },
                    Bus {
                        id: 13,
                        in_service: true,
                    },
                    Bus {
                        id: 0,
                        in_service: false,
                    },
                    Bus {
                        id: 0,
                        in_service: false,
                    },
                    Bus {
                        id: 59,
                        in_service: true,
                    },
                    Bus {
                        id: 0,
                        in_service: false,
                    },
                    Bus {
                        id: 31,
                        in_service: true,
                    },
                    Bus {
                        id: 19,
                        in_service: true,
                    },
                ],
            };

            assert_eq!(Schedule::from_str(input).unwrap(), expected);
        }

        #[test]
        fn earliest_departure() {
            let input = "7,13,x,x,59,x,31,19";
            let s = Schedule::from_str(input).unwrap();

            assert_eq!(
                s.earliest_departure(939),
                Some((
                    5,
                    Bus {
                        id: 59,
                        in_service: true
                    }
                ))
            );
        }

        #[test]
        fn sync_departures() {
            let input = "7,13,x,x,59,x,31,19";
            let s = Schedule::from_str(input).unwrap();
            assert_eq!(s.sync_departures(), Some(1068781));
        }

        #[test]
        fn foo() {
            for i in 0..1 {
                println!("-------------------------------------");
                let left = Slot(i + 4, Bus::new(59));
                let right = Slot(6, Bus::new(31));
                let pair = Pair { left, right };
                for intersect in pair.intersections(3) {
                    println!("{}", intersect);
                }
            }

            for i in 0..1 {
                println!("-------------------------------------");
                let left = Slot(i, Bus::new(7));
                let right = Slot(1, Bus::new(13));
                let pair = Pair { left, right };
                for intersect in pair.intersections(3) {
                    println!("{}", intersect);
                }
            }

            for i in 0..1 {
                println!("-------------------------------------");
                let left = Slot(i + 1, Bus::new(13));
                let right = Slot(4, Bus::new(59));
                let pair = Pair { left, right };
                for intersect in pair.intersections(3) {
                    println!("{}", intersect);
                }
            }

            for i in 0..1 {
                println!("-------------------------------------");
                let left = Slot(i, Bus::new(7));
                let right = Slot(6, Bus::new(31));
                let pair = Pair { left, right };
                for intersect in pair.intersections(3) {
                    println!("{}", intersect);
                }
            }

            for i in 0..1 {
                println!("-------------------------------------");
                let left = Slot(i, Bus::new(7));
                let right = Slot(7, Bus::new(19));
                let pair = Pair { left, right };
                for intersect in pair.intersections(3) {
                    println!("{}", intersect);
                }
            }

            // assert!(false);
        }

        #[derive(Debug)]
        struct Info<'a>(usize, &'a Bus);

        impl<'a> fmt::Display for Info<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.0, self.1.id())
            }
        }
    }
}
