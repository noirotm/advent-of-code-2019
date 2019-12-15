use crate::intcode::AsyncIO;
use crate::{
    intcode::{parse_program, IntCodeComputer},
    solver::Solver,
};
use std::{
    cmp::Ordering, collections::HashMap, io::Read, iter::repeat, sync::mpsc::Receiver, thread,
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = usize;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        parse_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (io, _tx, rx) = AsyncIO::new();
        let mut computer = IntCodeComputer::new(input.clone(), io);

        let handle = thread::spawn(move || computer.run());

        let mut screen = Screen {
            score: 0,
            cells: Default::default(),
        };

        while let Some((x, y, tile_id)) = recv_all(&rx) {
            if x == -1 {
                screen.score = tile_id;
            }
            screen.cells.insert(Pos { x, y }, Tile::from_i64(tile_id));
        }

        let _ = handle.join();
        screen
            .cells
            .values()
            .filter(|&v| v.eq(&Tile::Block))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (io, tx, rx) = AsyncIO::new();
        let mut program = input.clone();
        program[0] = 2; // play for free haxxxx
        let mut computer = IntCodeComputer::new(program, io);
        let handle = thread::spawn(move || computer.run());
        let mut screen = Screen {
            score: 0,
            cells: Default::default(),
        };

        let mut ball_x = 0;
        let mut paddle_x = 0;
        let mut started = false;

        while let Some((x, y, tile_id)) = recv_all(&rx) {
            if x == -1 && y == 0 {
                screen.score = tile_id;
                continue;
            }

            let tile = Tile::from_i64(tile_id);

            // identify paddle and ball
            match tile {
                Tile::Ball => ball_x = x,
                Tile::Paddle => paddle_x = x,
                _ => {}
            }
            screen.cells.insert(Pos { x, y }, Tile::from_i64(tile_id));

            if started {
                // program waits for input as soon as it printed a ball
                if tile == Tile::Ball {
                    // move paddle towards ball
                    tx.send(match paddle_x.cmp(&ball_x) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => -1,
                    })
                    .unwrap_or_else(|e| println!("Error: {}", e));
                }

            //screen.display();
            //thread::sleep(Duration::from_millis(10));
            } else if x == 39 && y == 19 {
                // effectively start game when all the field is drawn
                started = true;
                let _ = tx.send(0);
            }
        }

        let _ = handle.join();

        screen.score
    }
}

fn recv_all(rx: &Receiver<i64>) -> Option<(i64, i64, i64)> {
    Some((rx.recv().ok()?, rx.recv().ok()?, rx.recv().ok()?))
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_i64(n: i64) -> Self {
        match n {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!("wrong tile id"),
        }
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Block => '▭',
            Tile::Paddle => '_',
            Tile::Ball => '●',
        }
    }
}

struct Screen {
    score: i64,
    cells: HashMap<Pos, Tile>,
}

impl Screen {
    #[allow(dead_code)]
    fn display(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for pos in self.cells.keys() {
            if pos.x < min_x {
                min_x = pos.x;
            }
            if pos.x > max_x {
                max_x = pos.x;
            }
            if pos.y < min_y {
                min_y = pos.y;
            }
            if pos.y > max_y {
                max_y = pos.y;
            }
        }

        let w = max_x - min_x + 1;
        let h = max_y - min_y + 1;
        let x_offset = -min_x;
        let y_offset = -min_y;

        let mut canvas: Vec<Vec<char>> = repeat(repeat(' ').take(w as usize).collect())
            .take(h as usize)
            .collect();

        for (pt, tile) in self.cells.iter() {
            let x = (pt.x + x_offset) as usize;
            let y = (pt.y + y_offset) as usize;
            canvas[y][x] = tile.to_char();
        }

        println!("SCORE: {}", self.score);
        for row in canvas {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}
