use crate::error::{AocError, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Card(usize);

impl FromStr for Card {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Card(s.parse::<usize>()?))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Hand {
    winner: usize,
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(winner: usize, cards: Vec<Card>) -> Self {
        Hand {
            winner: winner,
            cards: cards,
        }
    }
}

impl FromIterator<(usize, Option<Card>)> for Hand {
    fn from_iter<I: IntoIterator<Item = (usize, Option<Card>)>>(iter: I) -> Self {
        let mut cards = Vec::new();

        let mut max_card = Card(0);
        let mut max_i = 0;
        for (i, card) in iter {
            match card {
                Some(card) => {
                    if card.0 > max_card.0 {
                        max_card = card;
                        max_i = i;
                    }
                    cards.push(card);
                }
                None => {}
            }
        }

        Hand::new(max_i, cards)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Player {
    name: String,
    deck: VecDeque<Card>,
}

impl Player {
    pub fn from_input(input: &[String]) -> Result<Self> {
        let mut parts = input.iter();
        if let Some(name) = parts.next() {
            let cards = parts
                .map(|c| Card::from_str(c))
                .collect::<Result<Vec<Card>>>()?;

            return Ok(Self::new(name.clone(), cards));
        }

        Err(AocError::InvalidInput(format!(
            "Could not make player from {:#?}",
            input
        )))
    }

    pub fn new(name: String, cards: Vec<Card>) -> Self {
        Player {
            name: name,
            deck: VecDeque::from(cards),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.deck.is_empty()
    }

    pub fn cards_remaining(&self) -> usize {
        self.deck.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.deck.pop_front()
    }

    pub fn peek(&self, n: usize) -> Vec<Card> {
        self.deck.iter().cloned().take(n).collect()
    }

    pub fn take(&mut self, hand: Hand) {
        for c in hand.cards {
            self.deck.push_back(c);
        }
    }

    pub fn score(&self) -> usize {
        let len = self.deck.len();
        self.deck
            .iter()
            .enumerate()
            .map(|(index, card)| (len - index) * card.0)
            .sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    players: Vec<Player>,
    round: usize,
}

impl Game {
    pub fn from_input(input: &[String]) -> Result<Self> {
        let players = input
            .split(|line| line.is_empty())
            .map(|player_input| Player::from_input(player_input))
            .collect::<Result<Vec<Player>>>()?;

        Ok(Game::new(players))
    }

    pub fn new(players: Vec<Player>) -> Self {
        Game {
            players: players,
            round: 0,
        }
    }

    pub fn play(&mut self) -> Result<(Player, usize)> {
        loop {
            let remaining = self
                .players
                .iter()
                .enumerate()
                .filter(|(_, p)| !p.is_empty())
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();

            if remaining.len() > 1 {
                self.round();
            } else {
                return match self.players.get(remaining[0]) {
                    Some(player) => Ok((player.clone(), remaining[0])),
                    None => Err(AocError::GameError("could not get winner".to_string())),
                };
            }
        }
    }

    pub fn round(&mut self) {
        self.round += 1;

        let cards = self
            .players
            .iter_mut()
            .enumerate()
            .map(|(i, p)| (i, p.draw()))
            .collect::<Vec<(usize, Option<Card>)>>();

        let mut h = Hand::from_iter(cards);
        h.cards.sort();
        h.cards.reverse();

        if let Some(player) = self.players.get_mut(h.winner) {
            player.take(h);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RecursiveGame {
    players: Vec<Player>,
    rounds: HashSet<Vec<usize>>,
}

impl RecursiveGame {
    pub fn from_input(input: &[String]) -> Result<Self> {
        let players = input
            .split(|line| line.is_empty())
            .map(|player_input| Player::from_input(player_input))
            .collect::<Result<Vec<Player>>>()?;

        Ok(RecursiveGame::new(players))
    }

    pub fn new(players: Vec<Player>) -> Self {
        RecursiveGame {
            players: players,
            rounds: HashSet::new(),
        }
    }

    pub fn play(&mut self) -> Result<(Player, usize)> {
        loop {
            // check if ever a round like this one -> player 1 wins
            let scores = self.players.iter().map(|p| p.score()).collect();
            if self.rounds.contains(&scores) {
                return match self.players.get(0) {
                    Some(player) => Ok((player.clone(), 0)),
                    None => Err(AocError::GameError("could not get winner".to_string())),
                };
            } else {
                self.rounds.insert(scores);
            }

            let remaining = self
                .players
                .iter()
                .enumerate()
                .filter(|(_, p)| !p.is_empty())
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();

            if remaining.len() > 1 {
                self.round()?;
            } else {
                return match self.players.get(remaining[0]) {
                    Some(player) => Ok((player.clone(), remaining[0])),
                    None => Err(AocError::GameError("could not get winner".to_string())),
                };
            }
        }
    }

    pub fn round(&mut self) -> Result<()> {
        let cards = self
            .players
            .iter_mut()
            .enumerate()
            .map(|(i, p)| {
                let c = p.draw();
                let cards_to_take = match c {
                    Some(c) if p.cards_remaining() >= c.0 => p.peek(c.0),
                    _ => Vec::new(),
                };

                (i, c, cards_to_take)
            })
            .collect::<Vec<(usize, Option<Card>, Vec<Card>)>>();

        let mut h = Hand {
            winner: 0,
            cards: Vec::new(),
        };

        // check if should recurse
        if cards
            .iter()
            .filter(|(_, _, cards_to_take)| !cards_to_take.is_empty())
            .count()
            > 1
        {
            // recurse
            let players = cards
                .iter()
                .map(|(i, _, cards_to_take)| Player::new(format!("{}", i), cards_to_take.clone()))
                .collect::<Vec<Player>>();

            let mut new_game = RecursiveGame::new(players);
            let winner = new_game.play()?;

            let winning_card = cards
                .iter()
                .filter(|(i, _, _)| *i == winner.1)
                .map(|(_, c, _)| c.unwrap())
                .next()
                .unwrap();

            h.cards.push(winning_card);
            cards
                .iter()
                .filter(|(_, c, _)| *c != Some(winning_card) && c.is_some())
                .for_each(|(_, c, _)| h.cards.push(c.unwrap()));

            h.winner = winner.1;
        } else {
            h = Hand::from_iter(cards.into_iter().map(|(i, c, _)| (i, c)));

            h.cards.sort();
            h.cards.reverse();
        }

        if let Some(player) = self.players.get_mut(h.winner) {
            player.take(h);
        }

        Ok(())
    }

    pub fn play_cached(&mut self) -> Result<(Player, usize)> {
        let mut cache = HashMap::new();
        self.play_c(&mut cache)
    }

    pub fn play_c(&mut self, cache: &mut HashMap<Vec<usize>, usize>) -> Result<(Player, usize)> {
        let initial_scores = self.players.iter().map(|p| p.score()).collect();
        if let Some(winner) = cache.get(&initial_scores) {
            return match self.players.get(*winner) {
                Some(player) => Ok((player.clone(), *winner)),
                None => Err(AocError::GameError("could not get winner".to_string())),
            };
        } else {
            let winner = loop {
                // check if ever a round like this one -> player 1 wins
                let scores = self.players.iter().map(|p| p.score()).collect();
                if self.rounds.contains(&scores) {
                    break match self.players.get(0) {
                        Some(player) => Ok((player.clone(), 0)),
                        None => Err(AocError::GameError("could not get winner".to_string())),
                    };
                } else {
                    self.rounds.insert(scores);
                }

                let remaining = self
                    .players
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.is_empty())
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>();

                if remaining.len() > 1 {
                    self.round_cached(cache)?;
                } else {
                    break match self.players.get(remaining[0]) {
                        Some(player) => Ok((player.clone(), remaining[0])),
                        None => Err(AocError::GameError("could not get winner".to_string())),
                    };
                }
            };

            if let Ok(ref winner) = winner {
                *cache.entry(initial_scores).or_insert(0) = winner.1;
            }

            winner
        }
    }

    pub fn round_cached(&mut self, cache: &mut HashMap<Vec<usize>, usize>) -> Result<()> {
        let cards = self
            .players
            .iter_mut()
            .enumerate()
            .map(|(i, p)| {
                let c = p.draw();
                let cards_to_take = match c {
                    Some(c) if p.cards_remaining() >= c.0 => p.peek(c.0),
                    _ => Vec::new(),
                };

                (i, c, cards_to_take)
            })
            .collect::<Vec<(usize, Option<Card>, Vec<Card>)>>();

        let mut h = Hand {
            winner: 0,
            cards: Vec::new(),
        };

        // check if should recurse
        if cards
            .iter()
            .filter(|(_, _, cards_to_take)| !cards_to_take.is_empty())
            .count()
            > 1
        {
            // recurse
            let players = cards
                .iter()
                .map(|(i, _, cards_to_take)| Player::new(format!("{}", i), cards_to_take.clone()))
                .collect::<Vec<Player>>();

            let mut new_game = RecursiveGame::new(players);
            let winner = new_game.play_c(cache)?;

            let winning_card = cards
                .iter()
                .filter(|(i, _, _)| *i == winner.1)
                .map(|(_, c, _)| c.unwrap())
                .next()
                .unwrap();

            h.cards.push(winning_card);
            cards
                .iter()
                .filter(|(_, c, _)| *c != Some(winning_card) && c.is_some())
                .for_each(|(_, c, _)| h.cards.push(c.unwrap()));

            h.winner = winner.1;
        } else {
            h = Hand::from_iter(cards.into_iter().map(|(i, c, _)| (i, c)));

            h.cards.sort();
            h.cards.reverse();
        }

        if let Some(player) = self.players.get_mut(h.winner) {
            player.take(h);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_input;

    mod player {
        use super::*;

        #[test]
        fn drawing() {
            let mut p = Player::new("Test Player".to_string(), vec![Card(3), Card(2), Card(10)]);

            assert_eq!(p.draw(), Some(Card(3)));
            assert_eq!(p.draw(), Some(Card(2)));
            assert_eq!(p.draw(), Some(Card(10)));
            assert_eq!(p.draw(), None);
        }

        #[test]
        fn taking() {
            let mut p = Player::new("Test Player".to_string(), vec![Card(3), Card(2), Card(10)]);

            let h = Hand::new(999, vec![Card(4), Card(5)]);

            p.take(h);

            let expected = Player::new(
                "Test Player".to_string(),
                vec![Card(3), Card(2), Card(10), Card(4), Card(5)],
            );

            assert_eq!(p, expected);
        }

        #[test]
        fn scoring() {
            let p = Player::new(
                "Test Player".to_string(),
                vec![
                    Card(3),
                    Card(2),
                    Card(10),
                    Card(6),
                    Card(8),
                    Card(5),
                    Card(9),
                    Card(4),
                    Card(7),
                    Card(1),
                ],
            );

            assert_eq!(p.score(), 306);
        }

        #[test]
        fn peeking() {
            let p = Player::new(
                "Test Player".to_string(),
                vec![
                    Card(3),
                    Card(2),
                    Card(10),
                    Card(6),
                    Card(8),
                    Card(5),
                    Card(9),
                    Card(4),
                    Card(7),
                    Card(1),
                ],
            );

            let peek = p.peek(3);

            assert_eq!(peek, vec![Card(3), Card(2), Card(10)]);
        }
    }

    mod game {
        use super::*;

        #[test]
        fn play() {
            let input = test_input(
                "
                Player 1:
                9
                2
                6
                3
                1

                Player 2:
                5
                8
                4
                7
                10
            ",
            );

            let mut g = Game::from_input(&input).unwrap();
            let winner = g.play().unwrap();
            assert_eq!(winner.0.name, "Player 2:".to_string());
            assert_eq!(winner.0.score(), 306);
        }
    }

    mod recursive_game {
        use super::*;

        #[test]
        fn play() {
            let input = test_input(
                "
                Player 1:
                9
                2
                6
                3
                1

                Player 2:
                5
                8
                4
                7
                10
            ",
            );

            let mut g = RecursiveGame::from_input(&input).unwrap();
            let winner = g.play().unwrap();
            assert_eq!(winner.0.name, "Player 2:".to_string());
            assert_eq!(winner.0.score(), 291);
        }

        #[test]
        fn play_cached() {
            let input = test_input(
                "
                Player 1:
                9
                2
                6
                3
                1

                Player 2:
                5
                8
                4
                7
                10
            ",
            );

            let mut g = RecursiveGame::from_input(&input).unwrap();
            let winner = g.play_cached().unwrap();
            assert_eq!(winner.0.name, "Player 2:".to_string());
            assert_eq!(winner.0.score(), 291);
        }
    }
}
