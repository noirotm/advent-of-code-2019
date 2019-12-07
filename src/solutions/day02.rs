use crate::intcode::{parse_program, IntCodeComputer, NoIO};
use crate::solver::Solver;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Vec<i64> {
        parse_program(r)
    }

    fn solve_first(&self, input: &Vec<i64>) -> i64 {
        let mut program = input.clone();
        program[1] = 12;
        program[2] = 2;
        let mut computer = IntCodeComputer::new(program, NoIO {});
        computer.run();
        computer.program[0]
    }

    fn solve_second(&self, input: &Vec<i64>) -> i64 {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut program = input.clone();
                program[1] = noun;
                program[2] = verb;
                let mut computer = IntCodeComputer::new(program, NoIO {});
                computer.run();
                let output = computer.program[0];

                if output == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        0
    }
}
