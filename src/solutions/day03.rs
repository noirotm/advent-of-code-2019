use crate::solver::Solver;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Read};
use std::iter::FromIterator;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<Instruction>, Vec<Instruction>);
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> (Vec<Instruction>, Vec<Instruction>) {
        let mut r = BufReader::new(r);

        let mut buffer = String::new();
        let _ = r.read_line(&mut buffer).expect("error reading first line");
        let v1 = instructions_from_str(&buffer);

        buffer = String::new();
        let _ = r.read_line(&mut buffer).expect("error reading second line");
        let v2 = instructions_from_str(&buffer);

        (v1, v2)
    }

    fn solve_first(&self, (i1, i2): &(Vec<Instruction>, Vec<Instruction>)) -> u64 {
        let p1 = points_from_instructions(&i1);
        let p2 = points_from_instructions(&i2);

        let p1_set: HashSet<Point> = HashSet::from_iter(p1.into_iter());
        let p2_set: HashSet<Point> = HashSet::from_iter(p2.into_iter());

        p1_set
            .intersection(&p2_set)
            .map(|p| p.manhattan_distance_to_orig())
            .min()
            .unwrap_or(0)
    }

    fn solve_second(&self, (i1, i2): &(Vec<Instruction>, Vec<Instruction>)) -> u64 {
        let p1 = points_from_instructions(&i1);
        let p2 = points_from_instructions(&i2);

        let p1_set: HashSet<Point> = HashSet::from_iter(p1.into_iter());
        let p2_set: HashSet<Point> = HashSet::from_iter(p2.into_iter());

        let inter = p1_set.intersection(&p2_set).collect::<Vec<_>>();

        inter
            .iter()
            .cloned()
            .map(|p| {
                let p1 = p1_set.get(p).expect("no point");
                let p2 = p2_set.get(p).expect("no point");
                (p1.steps + p2.steps) as u64
            })
            .min()
            .unwrap_or(0)
    }
}

fn instructions_from_str(s: &str) -> Vec<Instruction> {
    s.trim()
        .split(',')
        .map(|c| Instruction {
            dir: c[0..1].parse::<Dir>().expect("error parsing "),
            size: c[1..].parse::<isize>().expect("error parsing size"),
        })
        .collect::<Vec<_>>()
}

fn points_from_instructions(instrs: &[Instruction]) -> Vec<Point> {
    let mut v = vec![];
    let mut pt = Point {
        x: 0,
        y: 0,
        steps: 0,
    };
    for i in instrs {
        let mut points = points_from_orig(&pt, i);
        pt = points.last().expect("no last point").clone();
        v.append(&mut points);
    }

    v
}

fn points_from_orig(orig: &Point, i: &Instruction) -> Vec<Point> {
    let (dx, dy) = i.dir.dir();
    let mut pt = orig.clone();
    let mut points = vec![];
    for _ in 0..i.size {
        pt = Point {
            x: pt.x + dx as i64,
            y: pt.y + dy as i64,
            steps: pt.steps + 1,
        };
        points.push(pt.clone());
    }

    points
}

pub struct Instruction {
    dir: Dir,
    size: isize,
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn dir(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Eq)]
pub struct Point {
    x: i64,
    y: i64,
    steps: usize,
}

impl Point {
    fn manhattan_distance_to_orig(&self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
