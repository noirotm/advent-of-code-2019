use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};
use std::iter::successors;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<u64> {
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|l| l.parse()).collect()
    }

    fn solve_first(&self, input: &Vec<u64>) -> u64 {
        input.iter().cloned().map(mass_fuel).sum()
    }

    fn solve_second(&self, input: &Vec<u64>) -> u64 {
        input.iter().cloned().map(total_fuel_mass).sum()
    }
}

fn mass_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn total_fuel_mass(mass: u64) -> u64 {
    successors(Some(mass_fuel(mass)), |&m| Some(mass_fuel(m)))
        .take_while(|&m| m != 0)
        .sum()
}

#[allow(dead_code)]
fn total_fuel_mass_orig(mass: u64) -> u64 {
    let mut total = 0u64;
    let mut m = mass;
    while m != 0 {
        m = mass_fuel(m);
        total += m;
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::*;

    #[test]
    fn test_mass_fuel() {
        assert_eq!(mass_fuel(12), 2);
        assert_eq!(mass_fuel(14), 2);
        assert_eq!(mass_fuel(1969), 654);
        assert_eq!(mass_fuel(100_756), 33583);
        assert_eq!(mass_fuel(1), 0);
    }

    #[test]
    fn test_total_mass_fuel() {
        assert_eq!(total_fuel_mass(14), 2);
        assert_eq!(total_fuel_mass(1969), 966);
        assert_eq!(total_fuel_mass(100_756), 50346);
    }
}
