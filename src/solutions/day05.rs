use crate::{
    intcode::{parse_program, IntCodeComputer, IO},
    solver::Solver,
};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        parse_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut computer = IntCodeComputer::new(input.clone(), SimpleIO { val: 1 });
        computer.run();
        computer.io.val
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut computer = IntCodeComputer::new(input.clone(), SimpleIO { val: 5 });
        computer.run();
        computer.io.val
    }
}

struct SimpleIO {
    val: i64,
}

impl IO for SimpleIO {
    fn get(&self) -> i64 {
        self.val
    }

    fn put(&mut self, val: i64) {
        self.val = val;
    }
}
