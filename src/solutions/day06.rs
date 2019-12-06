use crate::solver::Solver;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(String, String)>;
    type Output1 = u64;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|l| {
                let mut i = l.split(')');
                (i.next().unwrap().to_string(), i.next().unwrap().to_string())
            })
            .collect::<Vec<_>>()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let state = State::from_vec(&input);

        let mut orbits = 0u64;
        for object in state.objects {
            let mut parent = state.parents.get(object);
            while let Some(p) = parent {
                //print!("{} -> ", p);
                orbits += 1;
                parent = state.parents.get(p);
            }
            //println!(";");
        }

        orbits
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let you = String::from("YOU");
        let san = String::from("SAN");
        let state = State::from_vec(&input);

        let from = state.parents(&you);
        let to = state.parents(&san);

        let mut n_from = 0;
        let mut n_to = 0;

        for (i_from, s_from) in from.iter().enumerate() {
            if let Some(i_to) = to.iter().position(|s_to| s_from == s_to) {
                n_from = i_from;
                n_to = i_to;
                break;
            }
        }

        n_from + n_to
    }
}

struct State<'a> {
    objects: HashSet<&'a String>,
    parents: HashMap<&'a String, &'a String>,
}

impl<'a> State<'a> {
    fn from_vec(v: &'a Vec<(String, String)>) -> Self {
        let mut objects = HashSet::new();
        let mut parents = HashMap::new();
        for (a, b) in v {
            objects.insert(a);
            objects.insert(b);
            parents.insert(b, a);
        }
        Self { objects, parents }
    }

    fn parents(&self, s: &'a String) -> Vec<&'a String> {
        let mut parents = vec![];
        let mut parent = self.parents.get(s);
        while let Some(p) = parent {
            parents.push(*p);
            parent = self.parents.get(p);
        }
        parents
    }
}
