use crate::error::{AocError, Result};
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cup(usize);

#[derive(Debug, Clone)]
pub struct Game {
    cups: NodeList,
}

impl Game {
    pub fn new(cups: Vec<Cup>, desired_len: usize) -> Self {
        Game {
            cups: NodeList::new(cups, desired_len),
        }
    }

    pub fn from_str_with_len(s: &str, desired_len: usize) -> Result<Self> {
        let cups = s.chars()
            .map(|ch| {
                ch.to_digit(10)
                    .ok_or_else(|| AocError::InvalidInput("could not convert character to int".to_string()))
                    .and_then(|v| Ok(Cup(v as usize)))
            })
            .collect::<Result<Vec<Cup>>>()?;
        Ok(Game::new(cups, desired_len))
    }

    pub fn round(&mut self) {
        let n = 3;

        let res = self.cups.take(n);
        let exclude = self.cups.cups(res.0, res.1);
        let insertion_point = self.cups.find(&exclude);
        self.cups.insert(insertion_point, res.0, res.1);
        self.cups.advance();
    }

    pub fn simulate(&mut self, n: usize) {
        for _ in 0..n {
            self.round();
        }
    }

    pub fn order(&self) -> Vec<Cup> {
        if let Some(one) = self.cups.get_node(&Cup(1)) {
            self.cups.cups(one.next, one.prev)
        } else {
            Vec::new()
        }
    }

    pub fn order_string(&self) -> String {
        self.order().iter().map(|c| c.0.to_string()).collect::<Vec<String>>().join("")
    }

    pub fn crappy_checksum(&self) -> u128 {
        if let Some(one) = self.cups.get_node(&Cup(1)) {
            if let Some(first) = self.cups.get_node_by_index(one.next) {
                if let Some(second) = self.cups.get_node_by_index(first.next) {
                    return (first.val.0 as u128) * (second.val.0 as u128);
                }
            }
        }
        0
    }
}

impl FromStr for Game {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        let cups = s.chars()
            .map(|ch| {
                ch.to_digit(10)
                    .ok_or_else(|| AocError::InvalidInput("could not convert character to int".to_string()))
                    .and_then(|v| Ok(Cup(v as usize)))
            })
            .collect::<Result<Vec<Cup>>>()?;
        let len = cups.len();
        Ok(Game::new(cups, len))
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Node {
    prev: usize,
    next: usize,
    val: Cup,
}

impl Node {
    pub fn new(prev: usize, next: usize, val: Cup) -> Self {
        Node {
            prev,
            next,
            val
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeList {
   nodes: Vec<Node>,
   cup_map: Vec<usize>,
   current: usize,
   min: Cup,
   max: Cup,
}

impl NodeList {
    pub fn new(cups: Vec<Cup>, desired_len: usize) -> Self {
        let mut nl = NodeList {
            nodes: Vec::with_capacity(desired_len),
            cup_map: Vec::new(),
            current: 0,
            max: Cup(0),
            min: cups.iter().cloned().min().unwrap_or(Cup(0)),
        };
        let max = cups.iter().cloned().max().unwrap_or(Cup(0));
        nl.cup_map = vec![10_000_000; desired_len + max.0];

        for (i, cup) in cups.into_iter().enumerate() {
            let prev = if i == 0 {
                desired_len - 1
            } else {
                i - 1
            };
            nl.nodes.push(Node::new(prev, i + 1, cup.clone()));
            nl.cup_map[cup.0 - 1] = i;
        }

        let existing_len = nl.nodes.len();

        if existing_len == desired_len {
            nl.nodes[desired_len - 1].next = 0;
            nl.max = max;
        } else {
            for i in 0..(desired_len - existing_len) {
                let next = if i == desired_len - existing_len - 1 {
                    0
                } else {
                    existing_len + i + 1
                };
                let cup = Cup(max.0 + 1 + i);
                nl.nodes.push(Node::new(existing_len + i - 1, next, cup.clone()));
                nl.cup_map[cup.0 - 1] = existing_len + i;
            }

            nl.max = nl.nodes[nl.nodes[0].prev].val.clone();
        }


        nl
    }

    pub fn get_node_by_index(&self, index: usize) -> Option<Node> {
        self.nodes.get(index).cloned()
    }

    pub fn get_node(&self, cup: &Cup) -> Option<Node> {
        Some(self.nodes[self.cup_map[cup.0 - 1]].clone())
    }

    pub fn find(&self, exclude: &Vec<Cup>) -> usize {
        let current = self.nodes[self.current].val.clone();
        let mut v = current.0;
        loop {
            let candidate = if v == 0 || v - 1 < self.min.0 {
                v = self.max.0;
                self.max
            } else {
                v -= 1;
                Cup(v)
            };

            if exclude.contains(&candidate) {
                continue;
            }

            let potential_index = self.cup_map[candidate.0 - 1];

            if potential_index == 10_000_000 {
                continue;
            }

            return potential_index;
        }
    }

    pub fn cups(&self, start: usize, end: usize) -> Vec<Cup> {
        let mut v = Vec::with_capacity(3);

        let mut cur = self.nodes[start].clone();
        v.push(cur.val);

        loop {
            if cur.next == end {
                cur = self.nodes[cur.next].clone();
                v.push(cur.val);
                break;
            }

            cur = self.nodes[cur.next].clone();
            v.push(cur.val)
        }

        v
    }

    pub fn advance(&mut self) {
        self.current = self.nodes[self.current].next;
    }

    pub fn take_past(&mut self, from: usize, n: usize) -> (usize, usize) {
        let next = self.nodes[from].next;

        let mut count = 0;

        let mut last = next;
        loop {
            if count >= n - 1 {
                break;
            }
            last = self.nodes[last].next;
            count += 1;
        }

        let end = self.nodes[last].clone();

        if let Some(prev) = self.nodes.get_mut(from) {
            prev.next = end.next;
        }

        if let Some(next) = self.nodes.get_mut(end.next) {
            next.prev = from;
        }

        (next, last)

    }

    pub fn take(&mut self, n: usize) -> (usize, usize) {
        self.take_past(self.current, n)
    }

    pub fn insert(&mut self, at: usize, start: usize, end: usize) {
        let next = self.nodes[at].next;

        if let Some(insert_point) = self.nodes.get_mut(at) {
            insert_point.next = start;
        }

        if let Some(next) = self.nodes.get_mut(next) {
            next.prev = end;
        }

        if let Some(start) = self.nodes.get_mut(start) {
            start.prev = at;
        }

        if let Some(end) = self.nodes.get_mut(end) {
            end.next = next;
        }
    }
}

impl fmt::Display for NodeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted: Vec<String> = self.nodes
            .iter()
            .enumerate()
            .map(|(i, n)| format!("({} | {} | {}, {})", n.val.0, i, n.prev, n.next))
            .collect();

        write!(f, "{}\n{:#?}", self.current, formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_input;

    mod node_list {
        use super::*;
        #[test]
        fn construction() {
            let nl = NodeList::new(vec![Cup(1), Cup(3), Cup(5), Cup(8), Cup(6), Cup(2)], 12);
            assert_eq!(nl.nodes.len(), 12);
            assert_eq!(nl.nodes[11].next, 0);

            let nl = NodeList::new(vec![Cup(1), Cup(3), Cup(5), Cup(8), Cup(6), Cup(2)], 6);
            assert_eq!(nl.nodes.len(), 6);
            assert_eq!(nl.nodes[5].next, 0);
        }


        #[test]
        fn take() {
            let mut nl = NodeList::new(vec![Cup(1), Cup(3), Cup(5), Cup(8), Cup(6), Cup(2)], 12);
            println!("start {}", nl);

            assert_eq!(nl.take(3), (1, 3));

            nl.insert(5, 1, 3);

            println!("after {}", nl);
        }

        #[test]
        fn cups() {
            let mut nl = NodeList::new(vec![Cup(1), Cup(3), Cup(5), Cup(8), Cup(6), Cup(2)], 12);
            assert_eq!(nl.cups(2, 4), vec![Cup(5), Cup(8), Cup(6)]);
        }

        #[test]
        fn find() {
            let mut nl = NodeList::new(vec![Cup(1), Cup(3), Cup(5), Cup(8), Cup(6), Cup(2)], 12);
            println!("start {}", nl);
            nl.current = 2;

            assert_eq!(nl.find(&vec![]), 1);
            assert_eq!(nl.find(&vec![Cup(3)]), 5);
            assert_eq!(nl.find(&vec![Cup(3), Cup(1), Cup(2)]), 11);
        }
    }

    mod game {
        use super::*;

        #[test]
        fn round() {
            let mut g = Game::from_str("389125467").unwrap();
            let res = g.order();
            assert_eq!(res, vec![Cup(2), Cup(5), Cup(4), Cup(6), Cup(7), Cup(3), Cup(8), Cup(9)]);


            //-- move 1 --
            //cups: (3) 8  9  1  2  5  4  6  7
            //pick up: 8, 9, 1
            //destination: 2

            g.round();
            let res = g.order();
            assert_eq!(res, vec![Cup(5), Cup(4), Cup(6), Cup(7), Cup(3), Cup(2), Cup(8), Cup(9)]);

            //-- move 2 --
            //cups:  3 (2) 8  9  1  5  4  6  7
            //pick up: 8, 9, 1
            //destination: 7

            g.round();
            let res = g.order();
            assert_eq!(res, vec![Cup(3), Cup(2), Cup(5), Cup(4), Cup(6), Cup(7), Cup(8), Cup(9)]);
            //-- move 3 --
            //cups:  3  2 (5) 4  6  7  8  9  1
            //pick up: 4, 6, 7
            //destination: 3

            g.round();
            let res = g.order();
            assert_eq!(res, vec![Cup(3), Cup(4), Cup(6), Cup(7), Cup(2), Cup(5), Cup(8), Cup(9)]);
            //-- move 4 --
            //cups:  7  2  5 (8) 9  1  3  4  6
            //pick up: 9, 1, 3
            //destination: 7

            g.round();
            let res = g.order();
            assert_eq!(res, vec![Cup(3), Cup(2), Cup(5), Cup(8), Cup(4), Cup(6), Cup(7), Cup(9)]);
            //-- move 5 --
            //cups:  3  2  5  8 (4) 6  7  9  1
            //pick up: 6, 7, 9
            //destination: 3
            //
            g.round();
            let res = g.order();
            println!("{}", g.cups);
            assert_eq!(res, vec![Cup(3), Cup(6), Cup(7), Cup(9), Cup(2), Cup(5), Cup(8), Cup(4)]);
            //-- move 6 --
            //cups:  9  2  5  8  4 (1) 3  6  7
            //pick up: 3, 6, 7
            //destination: 9

            g.round();
            let res = g.order();
            println!("{}", g.cups);
            assert_eq!(res, vec![Cup(9), Cup(3), Cup(6), Cup(7), Cup(2), Cup(5), Cup(8), Cup(4)]);
            //-- move 7 --
            //cups:  7  2  5  8  4  1 (9) 3  6
            //pick up: 3, 6, 7
            //destination: 8

            g.round();
            let res = g.order();
            assert_eq!(res, vec![Cup(9), Cup(2), Cup(5), Cup(8), Cup(3), Cup(6), Cup(7), Cup(4)]);
            //-- move 8 --
            //cups:  8  3  6  7  4  1  9 (2) 5
            //pick up: 5, 8, 3
            //destination: 1
            //
            g.round();
            let res = g.order();
            assert_eq!(res, vec![Cup(5), Cup(8), Cup(3), Cup(9), Cup(2), Cup(6), Cup(7), Cup(4)]);
            //-- move 9 --
            //cups:  7  4  1  5  8  3  9  2 (6)
            //pick up: 7, 4, 1
            //destination: 5

            g.round();
            let res = g.order();
            assert_eq!(res, vec![Cup(8), Cup(3), Cup(9), Cup(2), Cup(6), Cup(5), Cup(7), Cup(4)]);
            //-- move 10 --
            //cups: (5) 7  4  1  8  3  9  2  6
            //pick up: 7, 4, 1
            //destination: 3
            //
            //-- final --
            //cups:  5 (8) 3  7  4  1  9  2  6
        }

        #[test]
        fn order() {
            let g = Game::from_str("389125467").unwrap();
            let res = g.order();

            assert_eq!(res, vec![
                Cup(2),
                Cup(5),
                Cup(4),
                Cup(6),
                Cup(7),
                Cup(3),
                Cup(8),
                Cup(9),
            ]);
        }
        //92658374. If the crab were to complete all 100 moves, the order after cup 1 would be 67384529.
        #[test]
        fn simulate() {
            let mut g = Game::from_str("389125467").unwrap();

            g.simulate(10);
            assert_eq!(g.order_string().as_str(), "92658374");

            g.simulate(90);
            assert_eq!(g.order_string().as_str(), "67384529");
        }

        // #[test]
        // fn simulate_large() {
        //     let mut g = Game::from_str_with_len("389125467", 1_000_000).unwrap();
        //     assert_eq!(g.cups.nodes.len(), 1_000_000);
        //     assert_eq!(g.cups.nodes[999_999].val, Cup(1_000_000));

        //     g.simulate(10_000_000);
        //     assert_eq!(g.crappy_checksum(), 149_245_887_792);
        // }
    }
}

