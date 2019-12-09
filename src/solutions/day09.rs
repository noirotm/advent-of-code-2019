use crate::intcode::{parse_program, IntCodeComputer, IO};
use crate::solver::Solver;
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
        let program = input.clone();
        let io = VecIO::new(1);
        let mut computer = IntCodeComputer::new(program, io);
        computer.run();
        computer.io.output[0]
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let program = input.clone();
        let io = VecIO::new(2);
        let mut computer = IntCodeComputer::new(program, io);
        computer.run();
        computer.io.output[0]
    }
}

struct VecIO {
    input: Vec<i64>,
    output: Vec<i64>,
}

impl IO for VecIO {
    fn get(&mut self) -> i64 {
        self.input.pop().unwrap()
    }

    fn put(&mut self, val: i64) {
        self.output.push(val);
    }
}

impl VecIO {
    fn new(input: i64) -> Self {
        Self {
            input: vec![input],
            output: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09_01() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let io = VecIO { output: vec![] };
        let mut computer = IntCodeComputer::new(program, io);
        computer.run();
        assert_eq!(
            computer.io.output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_day09_02() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let io = VecIO { output: vec![] };
        let mut computer = IntCodeComputer::new(program, io);
        computer.run();
        assert_eq!(computer.io.output, vec![1_219_070_632_396_864]);
    }

    #[test]
    fn test_day09_03() {
        let program = vec![104, 1125899906842624, 99];
        let io = VecIO { output: vec![] };
        let mut computer = IntCodeComputer::new(program, io);
        computer.run();
        assert_eq!(computer.io.output, vec![1125899906842624]);
    }
}
