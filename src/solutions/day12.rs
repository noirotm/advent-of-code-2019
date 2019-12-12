use crate::solver::Solver;
use itertools::Itertools;
use num::integer::Integer;
use regex::Regex;
use std::{
    cmp::Ordering,
    error::Error,
    io::{BufRead, BufReader, Read},
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Moon>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|s| Position::from_str(&s))
            .map(Moon::new)
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut moons = input.clone();
        for _ in 0..1000 {
            step(&mut moons);
        }
        total_energy(&moons)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut moons = input.clone();
        let mut s = 0u64;

        let mut period_x = 0u64;
        let mut period_y = 0u64;
        let mut period_z = 0u64;

        loop {
            s += 1;
            step(&mut moons);

            if period_x == 0 && all_x_eq(input, &moons) {
                period_x = s;
            }
            if period_y == 0 && all_y_eq(input, &moons) {
                period_y = s;
            }
            if period_z == 0 && all_z_eq(input, &moons) {
                period_z = s;
            }

            if period_x != 0 && period_y != 0 && period_z != 0 {
                break;
            }
        }

        period_x.lcm(&period_y).lcm(&period_z)
    }
}

fn total_energy(moons: &[Moon]) -> u64 {
    moons.iter().map(|m| m.energy()).sum()
}

fn step(moons: &mut [Moon]) {
    update_all_velocities(moons);
    update_all_positions(moons);
}

fn update_all_velocities(moons: &mut [Moon]) {
    for v in (0..moons.len()).combinations(2) {
        let a = v[0];
        let b = v[1];
        let (s1, s2) = moons.split_at_mut(a + 1);
        let m1 = &mut s1[a];
        let m2 = &mut s2[b - a - 1];
        update_velocities(m1, m2);
    }
}

fn update_all_positions(moons: &mut [Moon]) {
    for moon in moons.iter_mut() {
        moon.update_position();
    }
}

fn update_velocities(m1: &mut Moon, m2: &mut Moon) {
    match m1.position.x.cmp(&m2.position.x) {
        Ordering::Less => {
            m1.velocity.dx += 1;
            m2.velocity.dx -= 1;
        }
        Ordering::Greater => {
            m1.velocity.dx -= 1;
            m2.velocity.dx += 1;
        }
        Ordering::Equal => {}
    }
    match m1.position.y.cmp(&m2.position.y) {
        Ordering::Less => {
            m1.velocity.dy += 1;
            m2.velocity.dy -= 1;
        }
        Ordering::Greater => {
            m1.velocity.dy -= 1;
            m2.velocity.dy += 1;
        }
        Ordering::Equal => {}
    }
    match m1.position.z.cmp(&m2.position.z) {
        Ordering::Less => {
            m1.velocity.dz += 1;
            m2.velocity.dz -= 1;
        }
        Ordering::Greater => {
            m1.velocity.dz -= 1;
            m2.velocity.dz += 1;
        }
        Ordering::Equal => {}
    }
}

fn all_x_eq(orig: &[Moon], moons: &[Moon]) -> bool {
    orig.iter().zip(moons).all(|(o, m)| o.eq_x(m))
}

fn all_y_eq(orig: &[Moon], moons: &[Moon]) -> bool {
    orig.iter().zip(moons).all(|(o, m)| o.eq_y(m))
}

fn all_z_eq(orig: &[Moon], moons: &[Moon]) -> bool {
    orig.iter().zip(moons).all(|(o, m)| o.eq_z(m))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>")?;
        let cap = re.captures(s).ok_or("no match")?;
        Ok(Self {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
            z: cap[3].parse()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Velocity {
    dx: i32,
    dy: i32,
    dz: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    fn new(position: Position) -> Self {
        Self {
            position,
            velocity: Velocity {
                dx: 0,
                dy: 0,
                dz: 0,
            },
        }
    }

    fn update_position(&mut self) {
        self.position.x += self.velocity.dx;
        self.position.y += self.velocity.dy;
        self.position.z += self.velocity.dz;
    }

    fn potential_energy(&self) -> u64 {
        self.position.x.abs() as u64 + self.position.y.abs() as u64 + self.position.z.abs() as u64
    }

    fn kinetic_energy(&self) -> u64 {
        self.velocity.dx.abs() as u64
            + self.velocity.dy.abs() as u64
            + self.velocity.dz.abs() as u64
    }

    fn energy(&self) -> u64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn eq_x(&self, other: &Self) -> bool {
        self.position.x == other.position.x && self.velocity.dx == other.velocity.dx
    }

    fn eq_y(&self, other: &Self) -> bool {
        self.position.y == other.position.y && self.velocity.dy == other.velocity.dy
    }

    fn eq_z(&self, other: &Self) -> bool {
        self.position.z == other.position.z && self.velocity.dz == other.velocity.dz
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_position() {
        let p = Position::from_str("<x=-1, y=0, z=2>");
        assert_eq!(p.unwrap(), Position { x: -1, y: 0, z: 2 });
    }
}
