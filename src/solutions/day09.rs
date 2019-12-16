use crate::intcode::AsyncIO;
use crate::{
    intcode::{parse_program, IntCodeComputer},
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
        let (io, tx, rx) = AsyncIO::new();
        let _ = tx.send(1);

        let mut computer = IntCodeComputer::new(input.to_vec(), io);
        computer.run();

        rx.recv().unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (io, tx, rx) = AsyncIO::new();
        let _ = tx.send(2);

        let mut computer = IntCodeComputer::new(input.to_vec(), io);
        computer.run();

        rx.recv().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::from_fn;

    fn assert_output_eq(program: &[i64], expected_output: &[i64]) {
        let (io, tx, rx) = AsyncIO::new();
        let _ = tx.send(1);
        let mut computer = IntCodeComputer::new(program.to_vec(), io);
        computer.run();
        assert_eq!(
            from_fn(|| rx.recv().ok()).collect::<Vec<_>>(),
            expected_output.to_vec(),
        );
    }

    #[test]
    fn test_01() {
        assert_output_eq(
            &[
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            &[
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
        );
    }

    #[test]
    fn test_02() {
        assert_output_eq(
            &[1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            &[1_219_070_632_396_864],
        );
    }

    #[test]
    fn test_03() {
        assert_output_eq(&[104, 1_125_899_906_842_624, 99], &[1_125_899_906_842_624]);
    }
}
