use crate::solver::Solver;
use itertools::{repeat_n, Itertools};
use std::{io::Read, iter::successors};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u8>;
    type Output1 = String;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.bytes().flatten().map(|b| b - b'0').collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let digits = successors(Some(next_phase(input)), |d| Some(next_phase(d)))
            .take(100)
            .last()
            .unwrap();

        digits[0..8].iter().copied().join("")
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let msg_offset = input[0..7]
            .iter()
            .copied()
            .join("")
            .parse()
            .unwrap_or(0usize);
        let size = input.len();

        let mut digits = input
            .iter()
            .cloned()
            .cycle()
            .take(10000 * size)
            .skip(msg_offset)
            .collect::<Vec<_>>();

        // This pattern only works on the 2nd half of the array
        for _ in 0..100 {
            for i in (0..digits.len() - 1).rev() {
                digits[i] = (digits[i] + digits[i + 1]) % 10;
            }
        }

        digits[0..8].iter().copied().join("")
    }
}

fn next_phase(digits: &[u8]) -> Vec<u8> {
    let n = digits.len();
    (0..n)
        .map(|i| {
            let s: i64 = digits
                .iter()
                .zip(pattern(i, n))
                .map(|(&a, b)| (a as i64 * b as i64))
                .sum();
            (s.abs() % 10) as u8
        })
        .collect()
}

fn pattern(i: usize, total: usize) -> Vec<i8> {
    let base = &[0, 1, 0, -1];
    base.iter()
        .flat_map(|&v| repeat_n(v, i + 1).collect::<Vec<_>>())
        .cycle()
        .skip(1)
        .take(total)
        .collect()
}

/*fn gen_all_patterns(size: usize) -> Vec<Vec<i8>> {
    (0..size).map(|n| pattern(n, size)).collect()
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {
        assert_eq!(pattern(2, 12), vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0]);
        assert_eq!(pattern(0, 8), vec![1, 0, -1, 0, 1, 0, -1, 0]);
    }

    #[test]
    fn test_next_phase() {
        let phase = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let next = vec![4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(next_phase(&phase), next);
    }
}
