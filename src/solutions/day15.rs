use crate::{
    intcode::{parse_program, AsyncIO, IntCodeComputer},
    solver::Solver,
};
use itertools::repeat_n;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryFrom,
    fmt::{Display, Error, Formatter},
    io::Read,
    sync::mpsc::{Receiver, Sender},
    thread::spawn,
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        parse_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (io, tx, rx) = AsyncIO::new();
        let mut computer = IntCodeComputer::new(input.to_vec(), io);

        let t = spawn(move || computer.run());
        let map = build_map(&tx, &rx);
        drop(tx);
        let _ = t.join();

        find_steps_from_origin(&map)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (io, tx, rx) = AsyncIO::new();
        let mut computer = IntCodeComputer::new(input.to_vec(), io);

        let t = spawn(move || computer.run());
        let map = build_map(&tx, &rx);
        drop(tx);
        let _ = t.join();

        oxygen_fill(&map)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Oxygen,
    Unknown,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(c: char) -> Result<Cell, ()> {
        match c {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Wall),
            'O' => Ok(Cell::Oxygen),
            ' ' => Ok(Cell::Unknown),
            _ => Err(()),
        }
    }
}

impl From<&Cell> for char {
    fn from(c: &Cell) -> Self {
        match c {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Oxygen => 'O',
            Cell::Unknown => ' ',
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::East => Self::North,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::East => Self::South,
        }
    }
}

impl TryFrom<i64> for Dir {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Dir::North),
            2 => Ok(Dir::South),
            3 => Ok(Dir::West),
            4 => Ok(Dir::West),
            _ => Err(()),
        }
    }
}

impl From<&Dir> for i64 {
    fn from(d: &Dir) -> Self {
        match d {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn point_in_dir(&self, dir: &Dir) -> Point {
        match dir {
            Dir::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Dir::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Dir::West => Point {
                x: self.x - 1,
                y: self.y,
            },
            Dir::East => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

struct Map {
    rows: Vec<Vec<Cell>>,
    w: usize,
    h: usize,
    origin: Point,
}

impl Map {
    fn from_points_map(points: &HashMap<Point, Cell>) -> Self {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for pt in points.keys() {
            if pt.x < min_x {
                min_x = pt.x;
            }
            if pt.x > max_x {
                max_x = pt.x;
            }
            if pt.y < min_y {
                min_y = pt.y;
            }
            if pt.y > max_y {
                max_y = pt.y;
            }
        }
        let w = max_x - min_x + 1;
        let h = max_y - min_y + 1;
        let x_offset = -min_x;
        let y_offset = -min_y;
        let origin = Point {
            x: x_offset,
            y: y_offset,
        };

        let mut map = Self::new(w as usize, h as usize, origin);
        for (pt, cell) in points {
            let x = (pt.x + x_offset) as usize;
            let y = (pt.y + y_offset) as usize;
            map.set(x, y, cell.clone());
        }

        map
    }

    fn new(w: usize, h: usize, origin: Point) -> Self {
        Self {
            rows: repeat_n(repeat_n(Cell::Unknown, w).collect(), h).collect(),
            w,
            h,
            origin,
        }
    }

    fn set(&mut self, x: usize, y: usize, value: Cell) {
        self.rows[y][x] = value;
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        self.rows.get(y)?.get(x).cloned()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in self.rows.iter() {
            for cell in row {
                write!(f, "{}", char::from(cell))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn build_map(tx: &Sender<i64>, rx: &Receiver<i64>) -> Map {
    let mut visited = HashSet::new();
    let mut points = HashMap::new();
    let mut cur_pos = Point { x: 0, y: 0 };
    let mut cur_dir = Dir::East;

    points.insert(cur_pos.clone(), Cell::Empty);

    while !visited.contains(&(cur_pos.clone(), cur_dir.clone())) {
        // insert current pos / dir
        visited.insert((cur_pos.clone(), cur_dir.clone()));

        // try to advance in the same direction
        let status = robot_turn(&cur_dir, tx, rx);

        match status {
            0 => {
                // found a wall
                points.insert(cur_pos.point_in_dir(&cur_dir), Cell::Wall);
                cur_dir = cur_dir.turn_left();
            }
            1 => {
                // open path, turn right
                points.insert(cur_pos.point_in_dir(&cur_dir), Cell::Empty);
                cur_pos = cur_pos.point_in_dir(&cur_dir);
                cur_dir = cur_dir.turn_right();
            }
            2 => {
                // oxygen system, turn right
                points.insert(cur_pos.point_in_dir(&cur_dir), Cell::Oxygen);
                cur_pos = cur_pos.point_in_dir(&cur_dir);
                cur_dir = cur_dir.turn_right();
            }
            _ => unreachable!(),
        }
    }

    Map::from_points_map(&points)
}

fn robot_turn(dir: &Dir, tx: &Sender<i64>, rx: &Receiver<i64>) -> i64 {
    let _ = tx.send(dir.into());
    rx.recv().unwrap()
}

fn find_steps_from_origin(map: &Map) -> u64 {
    // dijkstra !
    let mut unvisited = HashSet::new();
    for y in 0..map.h {
        for x in 0..map.w {
            unvisited.insert((x, y));
        }
    }

    let mut oxygen_system_cost = std::u64::MAX;

    let mut queue = VecDeque::from(vec![(map.origin.clone(), 0u64)]);

    while let Some((point, cost)) = queue.pop_front() {
        let pt = (point.x as usize, point.y as usize);
        if unvisited.contains(&pt) {
            unvisited.remove(&pt);

            match map.get(point.x as usize, point.y as usize) {
                Some(Cell::Empty) => {
                    // visit neighbours and increment cost
                    let n = point.point_in_dir(&Dir::North);
                    let s = point.point_in_dir(&Dir::South);
                    let e = point.point_in_dir(&Dir::East);
                    let w = point.point_in_dir(&Dir::West);
                    queue.push_back((n, cost + 1));
                    queue.push_back((s, cost + 1));
                    queue.push_back((e, cost + 1));
                    queue.push_back((w, cost + 1));
                }
                Some(Cell::Wall) | Some(Cell::Unknown) => {
                    // infinite cost, don't visit neighbours
                }
                Some(Cell::Oxygen) => {
                    // add cost, but stop here
                    oxygen_system_cost = cost;
                    break;
                }
                None => {} // outside the grid, ignore
            }
        }
    }

    // we've found the oxygen system, we've also got its minimum cost!
    oxygen_system_cost
}

fn oxygen_fill(map: &Map) -> u64 {
    // dijkstra !
    let mut unvisited = HashSet::new();
    let mut origin = Point { x: 0, y: 0 };
    for y in 0..map.h {
        for x in 0..map.w {
            unvisited.insert((x, y));
            if map.get(x, y) == Some(Cell::Oxygen) {
                origin = Point {
                    x: x as i64,
                    y: y as i64,
                };
            }
        }
    }

    let mut cost_grid: Vec<Vec<u64>> =
        repeat_n(repeat_n(std::u64::MAX, map.w).collect(), map.h).collect();

    let mut queue = VecDeque::from(vec![(origin, 0u64)]);

    while let Some((point, cost)) = queue.pop_front() {
        let pt = (point.x as usize, point.y as usize);
        if unvisited.contains(&pt) {
            unvisited.remove(&pt);

            match map.get(point.x as usize, point.y as usize) {
                Some(Cell::Empty) | Some(Cell::Oxygen) => {
                    cost_grid[point.y as usize][point.x as usize] = cost;

                    // visit neighbours and increment cost
                    let n = point.point_in_dir(&Dir::North);
                    let s = point.point_in_dir(&Dir::South);
                    let e = point.point_in_dir(&Dir::East);
                    let w = point.point_in_dir(&Dir::West);
                    queue.push_back((n, cost + 1));
                    queue.push_back((s, cost + 1));
                    queue.push_back((e, cost + 1));
                    queue.push_back((w, cost + 1));
                }
                Some(Cell::Wall) | Some(Cell::Unknown) => {
                    // infinite cost, don't visit neighbours
                    cost_grid[point.y as usize][point.x as usize] = std::u64::MAX;
                }
                None => {} // outside the grid, ignore
            }
        }
    }

    // we've got all costs from oxygen system, find the max
    cost_grid
        .iter()
        .map(|row| {
            *row.iter()
                .filter(|&&c| c != std::u64::MAX)
                .max()
                .unwrap_or(&0)
        })
        .max()
        .unwrap()
}
