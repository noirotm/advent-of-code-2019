use crate::intcode::AsyncIO;
use crate::{
    intcode::{parse_program, IntCodeComputer},
    solver::Solver,
};
use std::{
    collections::HashMap,
    error::Error,
    io::Read,
    iter::repeat,
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = usize;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        parse_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut panel = Panel::new();
        let mut robot = Robot::new(input.clone());

        loop {
            if robot.paint(&mut panel).is_err() {
                break;
            }
        }
        robot.wait();

        panel.points.len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut panel = Panel::new();
        panel.paint(&Point { x: 0, y: 0 }, &Color::White);

        let mut robot = Robot::new(input.clone());

        loop {
            if robot.paint(&mut panel).is_err() {
                break;
            }
        }
        robot.wait();
        panel.display();

        String::from("GARPKZUL")
    }
}

struct Panel {
    points: HashMap<Point, Color>,
}

impl Panel {
    fn new() -> Self {
        Self {
            points: Default::default(),
        }
    }

    fn paint(&mut self, p: &Point, c: &Color) {
        self.points.insert(p.clone(), c.clone());
    }

    fn color(&self, p: &Point) -> Color {
        self.points.get(p).cloned().unwrap_or(Color::Black)
    }

    fn display(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for pt in self.points.keys() {
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

        let mut canvas: Vec<Vec<char>> = repeat(repeat('.').take(w as usize).collect())
            .take(h as usize)
            .collect();

        for (pt, color) in self.points.iter() {
            let x = (pt.x + x_offset) as usize;
            let y = (pt.y + y_offset) as usize;
            canvas[y][x] = color.to_char();
        }

        for row in canvas {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Color {
    Black,
    White,
}

impl Color {
    fn from_i64(i: i64) -> Self {
        match i {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("invalid color"),
        }
    }

    fn to_i64(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Black => '.',
            Self::White => '#',
        }
    }
}

#[derive(Clone, Debug)]
enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn from_i64(dir: i64) -> Self {
        match dir {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct Robot {
    position: Point,
    direction: (isize, isize),
    tx: Sender<i64>,
    rx: Receiver<i64>,
    handle: JoinHandle<()>,
}

impl Robot {
    fn new(program: Vec<i64>) -> Self {
        let (io, tx, rx) = AsyncIO::new();
        let mut computer = IntCodeComputer::new(program, io);

        let handle = thread::spawn(move || computer.run());

        Self {
            position: Point { x: 0, y: 0 },
            direction: (0, -1),
            tx,
            rx,
            handle,
        }
    }

    fn wait(self) {
        let _ = self.handle.join();
    }

    fn paint(&mut self, panel: &mut Panel) -> Result<(), Box<dyn Error>> {
        // color at current position in panel
        let current_color = panel.color(&self.position);

        // send error means program has exited, abort then
        self.tx.send(current_color.to_i64())?;

        // wait for program to return new color, then turn direction, recv error means
        // program has exited, abort then
        let color = Color::from_i64(self.rx.recv()?);
        let turn_direction = TurnDirection::from_i64(self.rx.recv()?);

        // do paint the panel at current position with new color
        panel.paint(&self.position, &color);

        // turn and advance
        match turn_direction {
            TurnDirection::Left => self.turn_left(),
            TurnDirection::Right => self.turn_right(),
        }
        self.advance();

        // continue
        Ok(())
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            d => d,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            d => d,
        }
    }

    fn advance(&mut self) {
        let (dx, dy) = self.direction;
        self.position.x += dx;
        self.position.y += dy;
    }
}
