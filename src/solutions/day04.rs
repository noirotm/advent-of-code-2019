use crate::solver::Solver;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = (u32, u32);
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, mut r: R) -> Self::Input {
        let mut s = String::new();
        let _ = r.read_to_string(&mut s);
        let v = s.split('-').flat_map(|s| s.parse()).collect::<Vec<_>>();

        (v[0], v[1])
    }

    fn solve_first(&self, (a, b): &Self::Input) -> Self::Output1 {
        (*a..=*b).filter(|&n| is_number_ok_for_first(n)).count()
    }

    fn solve_second(&self, (a, b): &Self::Input) -> Self::Output2 {
        (*a..=*b).filter(|&n| is_number_ok_for_second(n)).count()
    }
}

fn is_number_ok_for_first(n: u32) -> bool {
    let b = n.to_string().into_bytes();
    let mut has_dup = false;

    for b in b.windows(2) {
        if b[0] > b[1] {
            return false;
        }

        if b[0] == b[1] {
            has_dup = true;
        }
    }

    has_dup
}

fn is_number_ok_for_second(n: u32) -> bool {
    let v = n.to_string().into_bytes();
    let mut has_dup = false;

    let mut i = 0;
    while i < v.len() {
        let b0 = *v.get(i).unwrap_or(&0);
        let b1 = *v.get(i + 1).unwrap_or(&0);
        let b2 = *v.get(i + 2).unwrap_or(&0);

        if b1 != 0 && b0 > b1 {
            return false;
        }

        if b0 == b1 {
            if b0 != b2 {
                has_dup = true;
                i += 1;
            } else {
                while *v.get(i + 1).unwrap_or(&0) == b0 {
                    i += 1;
                }
            }
        } else {
            i += 1;
        }
    }

    has_dup
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number_ok_for_first() {
        assert!(is_number_ok_for_first(111111));
        assert!(is_number_ok_for_first(122345));
        assert!(!is_number_ok_for_first(223450));
        assert!(!is_number_ok_for_first(123789));
    }

    #[test]
    fn test_is_number_ok_for_second() {
        assert!(is_number_ok_for_second(112233));
        assert!(is_number_ok_for_second(111122));
        assert!(!is_number_ok_for_second(123444));
    }
}
