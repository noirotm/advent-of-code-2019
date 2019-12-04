use crate::solver::Solver;
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader, Read},
    iter::FromIterator,
    str::FromStr,
};

pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<Point>, Vec<Point>);
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut lines = BufReader::new(r).lines().flatten();

        let l = lines.next().expect("error reading first line");
        let v1 = str_to_instructions(&l);
        let v1 = instructions_to_points(&v1);

        let l = lines.next().expect("error reading second line");
        let v2 = str_to_instructions(&l);
        let v2 = instructions_to_points(&v2);

        (v1, v2)
    }

    fn solve_first(&self, (p1, p2): &Self::Input) -> Self::Output1 {
        let p1_set: HashSet<Point> = HashSet::from_iter(p1.clone().into_iter());
        let p2_set: HashSet<Point> = HashSet::from_iter(p2.clone().into_iter());

        p1_set
            .intersection(&p2_set)
            .map(|p| p.manhattan_distance_to_orig())
            .min()
            .unwrap_or(0)
    }

    fn solve_second(&self, (p1, p2): &Self::Input) -> Self::Output2 {
        let p1_set: HashSet<Point> = HashSet::from_iter(p1.clone().into_iter());
        let p2_set: HashSet<Point> = HashSet::from_iter(p2.clone().into_iter());

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

fn str_to_instructions(s: &str) -> Vec<Instruction> {
    s.split(',')
        .map(|c| Instruction {
            dir: c[0..1].parse::<Dir>().expect("error parsing dir"),
            size: c[1..].parse::<isize>().expect("error parsing size"),
        })
        .collect::<Vec<_>>()
}

fn instructions_to_points(instructions: &[Instruction]) -> Vec<Point> {
    let mut v = vec![];
    let mut pt = Point {
        x: 0,
        y: 0,
        steps: 0,
    };
    for i in instructions {
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
