use crate::solver::Solver;
use regex::Regex;
use std::{
    io::{BufRead, BufReader, Read},
    iter::{repeat, FromIterator},
    str::FromStr,
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Shuffle>;
    type Output1 = usize;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|l| Shuffle::from_str(&l))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut deck = Deck::new(10007);
        for s in input.iter().cloned() {
            deck.shuffle(s);
        }

        deck.cards.iter().position(|&e| e == 2019).unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let deck = Deck::new(119_315_717_514_047);
        0
    }
}

#[derive(Clone)]
pub enum Shuffle {
    DealIntoNewStack,
    Cut(isize),
    DealWithIncrement(usize),
}

impl FromStr for Shuffle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cut_re = Regex::new(r"cut (-?\d+)").unwrap();
        let inc_re = Regex::new(r"deal with increment (\d+)").unwrap();

        if s == "deal into new stack" {
            Ok(Shuffle::DealIntoNewStack)
        } else if let Some(cap) = cut_re.captures(s) {
            let n = cap[1]
                .parse()
                .map_err(|e| format!("Invalid number: {}", e))?;
            Ok(Shuffle::Cut(n))
        } else if let Some(cap) = inc_re.captures(s) {
            let n = cap[1]
                .parse()
                .map_err(|e| format!("Invalid number: {}", e))?;
            Ok(Shuffle::DealWithIncrement(n))
        } else {
            Err("Invalid line".to_string())
        }
    }
}

struct Deck {
    cards: Vec<usize>,
}

impl Deck {
    fn new(size: usize) -> Self {
        Self {
            cards: Vec::from_iter(0..size),
        }
    }

    fn shuffle(&mut self, operation: Shuffle) {
        match operation {
            Shuffle::DealIntoNewStack => self.cards.reverse(),
            Shuffle::Cut(n) if n > 0 => self.cards.rotate_left(n as usize),
            Shuffle::Cut(n) if n < 0 => self.cards.rotate_right(-n as usize),
            Shuffle::DealWithIncrement(n) => self.deal_with_increment(n),
            _ => {}
        }
    }

    fn deal_with_increment(&mut self, inc: usize) {
        let mut idx = 0;
        let mut out = Vec::from_iter(repeat(0).take(self.cards.len()));
        for card in self.cards.iter() {
            out[idx] = *card;
            idx += inc;
            if idx >= out.len() {
                idx -= out.len();
            }
        }
        self.cards = out;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_into_new_stack() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::DealIntoNewStack);
        assert_eq!(deck.cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut_positive() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Cut(3));
        assert_eq!(deck.cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Cut(-4));
        assert_eq!(deck.cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::DealWithIncrement(3));
        assert_eq!(deck.cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_combine_01() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::DealWithIncrement(7));
        deck.shuffle(Shuffle::DealIntoNewStack);
        deck.shuffle(Shuffle::DealIntoNewStack);
        assert_eq!(deck.cards, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_combine_02() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Cut(6));
        deck.shuffle(Shuffle::DealIntoNewStack);
        deck.shuffle(Shuffle::DealIntoNewStack);
        assert_eq!(deck.cards, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }
}
