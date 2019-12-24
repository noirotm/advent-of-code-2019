use crate::{
    grid::Grid,
    intcode::{parse_program, AsyncIO, IntCodeComputer},
    solver::Solver,
};
use itertools::Itertools;
use std::{
    convert::TryFrom,
    fmt::{Display, Error, Formatter},
    io::Read,
    iter::from_fn,
    str::FromStr,
    thread::spawn,
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = usize;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        parse_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (io, tx, rx) = AsyncIO::new();
        let mut computer = IntCodeComputer::new(input.to_vec(), io);

        let t = spawn(move || computer.run());

        let s = String::from_utf8(
            from_fn(|| rx.recv().ok())
                .map(|v| v as u8)
                .collect::<Vec<_>>(),
        )
        .unwrap();

        drop(tx);
        let _ = t.join();

        let grid: Grid<Cell> = Grid::from_str(&s).unwrap();

        println!("{}", grid);

        (1..grid.w - 1)
            .cartesian_product(1..grid.h - 1)
            .filter(|(x, y)| is_intersection(&grid, (*x, *y)))
            .map(|(x, y)| x * y)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        0
    }
}

fn is_intersection(grid: &Grid<Cell>, (x, y): (usize, usize)) -> bool {
    (
        grid.get((x, y)),
        grid.get((x - 1, y)),
        grid.get((x + 1, y)),
        grid.get((x, y - 1)),
        grid.get((x, y + 1)),
    ) == (
        Some(&Cell::Wall),
        Some(&Cell::Wall),
        Some(&Cell::Wall),
        Some(&Cell::Wall),
        Some(&Cell::Wall),
    )
}

#[derive(Clone, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Bot(Dir),
}

impl TryFrom<u8> for Cell {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Cell::Empty),
            b'#' => Ok(Cell::Wall),
            b'^' => Ok(Cell::Bot(Dir::N)),
            b'v' => Ok(Cell::Bot(Dir::S)),
            b'>' => Ok(Cell::Bot(Dir::E)),
            b'<' => Ok(Cell::Bot(Dir::W)),
            v => Err(format!("Invalid cell: {}", v)),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => '.',
                Cell::Wall => '#',
                Cell::Bot(Dir::N) => '^',
                Cell::Bot(Dir::S) => 'v',
                Cell::Bot(Dir::E) => '>',
                Cell::Bot(Dir::W) => '<',
            }
        )
    }
}
