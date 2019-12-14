use crate::solver::Solver;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    io::{BufRead, BufReader, Read},
};

pub struct Problem;

impl Solver for Problem {
    type Input = HashMap<String, Reaction>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|s| parse_reaction(&s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut factory = Factory::new(input.clone());
        factory.build_element("FUEL", 1);
        factory.ore_used
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut factory = Factory::new(input.clone());
        let mut n_fuel = 0;

        while factory.build_element("FUEL", 1) {
            n_fuel += 1;
        }

        n_fuel
    }
}

#[derive(Clone, Debug)]
pub struct Reaction {
    output: u64,
    inputs: Vec<(String, u64)>,
}

fn parse_reaction(s: &str) -> (String, Reaction) {
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    let components = re
        .captures_iter(s)
        .map(|c| {
            let n = c[1].parse().unwrap_or(0);
            let elem = c[2].to_string();
            (elem, n)
        })
        .collect::<Vec<_>>();
    let ((elem, n), inputs) = components.split_last().unwrap();
    let reaction = Reaction {
        output: *n,
        inputs: inputs.to_vec(),
    };

    (elem.clone(), reaction)
}

#[derive(Debug)]
struct Factory {
    reactions: HashMap<String, Reaction>,
    inventory: HashMap<String, u64>,
    ore_used: u64,
}

impl Factory {
    fn new(reactions: HashMap<String, Reaction>) -> Self {
        let mut inventory = HashMap::new();
        inventory.insert(String::from("ORE"), 1_000_000_000_000);

        Self {
            reactions,
            inventory,
            ore_used: 0,
        }
    }

    fn build_element(&mut self, element: &str, amount: u64) -> bool {
        let mut build_queue = VecDeque::new();
        build_queue.push_back((element.to_string(), amount));

        while let Some((element, amount)) = build_queue.pop_front() {
            // what can we take from inventory
            let available = self.consume_from_inventory(&element, amount);
            let needed = amount - available;

            // nothing needed, continue
            if needed == 0 {
                continue;
            }

            // otherwise we need to add reactions to our queue until we can produce something
            if let Some(reaction) = self.reactions.get(&element).cloned() {
                let n_reactions_needed = Self::reactions_needed(needed, reaction.output);
                let actual_output = reaction.output * n_reactions_needed;
                let leftover = actual_output - needed;

                self.put_in_inventory(&element, leftover);

                for (input, amount) in reaction.inputs.iter() {
                    build_queue.push_back((input.clone(), amount * n_reactions_needed));
                }
            } else {
                return false; // no can do, no reaction to build this
            }
        }

        true
    }

    fn reactions_needed(needed: u64, reaction_output: u64) -> u64 {
        let mut n = 0;
        while needed > n * reaction_output {
            n += 1;
        }
        n
    }

    fn consume_from_inventory(&mut self, element: &str, requested_amount: u64) -> u64 {
        // special case, ore, we have as much as we want
        if element.eq("ORE") {
            self.ore_used += requested_amount;
            //return requested_amount;
        }

        if let Some(remaining_amount) = self.inventory.get_mut(element) {
            if *remaining_amount >= requested_amount {
                *remaining_amount -= requested_amount;
                requested_amount
            } else {
                let amount = *remaining_amount;
                *remaining_amount = 0;
                amount
            }
        } else {
            0
        }
    }

    fn put_in_inventory(&mut self, element: &str, amount: u64) {
        let a = self.inventory.entry(element.to_string()).or_insert(0);
        *a += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needed() {
        assert_eq!(Factory::reactions_needed(1, 1), 1);
        assert_eq!(Factory::reactions_needed(4, 2), 2);
        assert_eq!(Factory::reactions_needed(5, 5), 1);
        assert_eq!(Factory::reactions_needed(28, 10), 3);
        assert_eq!(Factory::reactions_needed(12, 17), 1);
        assert_eq!(Factory::reactions_needed(6, 5), 2);
    }
}
