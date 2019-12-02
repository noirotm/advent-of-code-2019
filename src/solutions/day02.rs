use crate::solver::Solver;
use std::io::{BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Vec<usize> {
        BufReader::new(r)
            .split(b',')
            .flatten()
            .flat_map(String::from_utf8)
            .flat_map(|s| s.parse())
            .collect()
    }

    fn solve_first(&self, input: &Vec<usize>) -> usize {
        let mut program = input.clone();
        program[1] = 12;
        program[2] = 2;
        let _ = run_program(&mut program);
        program[0]
    }

    fn solve_second(&self, input: &Vec<usize>) -> usize {
        for noun in 0..=99usize {
            for verb in 0..=99usize {
                let mut program = input.clone();
                program[1] = noun;
                program[2] = verb;
                let _ = run_program(&mut program);
                let output = program[0];

                if output == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        0
    }
}

fn run_program(program: &mut Vec<usize>) -> Result<(), String> {
    let mut ip = 0usize;
    loop {
        let opcode = program[ip];
        match opcode {
            1 => {
                let param1 = program[ip + 1];
                let param2 = program[ip + 2];
                let dest = program[ip + 3];
                program[dest] = program[param1] + program[param2];
                ip += 4;
            }
            2 => {
                let param1 = program[ip + 1];
                let param2 = program[ip + 2];
                let dest = program[ip + 3];
                program[dest] = program[param1] * program[param2];
                ip += 4;
            }
            99 => return Ok(()),
            o => return Err(format!("Invalid opcode: {}", o)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day02::run_program;

    fn assert_program_output_eq(mut program: Vec<usize>, output: Vec<usize>) {
        let res = run_program(&mut program);
        assert_eq!(res, Ok(()));
        assert_eq!(program, output);
    }

    #[test]
    fn test_run_program() {
        assert_program_output_eq(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
        assert_program_output_eq(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
        assert_program_output_eq(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
        assert_program_output_eq(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
