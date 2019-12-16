use crate::{
    intcode::{self, parse_program, AsyncIO, Connector, IntCodeComputer},
    solver::Solver,
};
use itertools::Itertools;
use std::{io::Read, iter::from_fn, sync::mpsc::channel, thread::spawn};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        parse_program(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        (0..5)
            .permutations(5)
            .map(|phases| run_with_phases(input, &phases))
            .max()
            .unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        (5..10)
            .permutations(5)
            .map(|phases| run_with_phases_async(input, &phases))
            .max()
            .unwrap()
    }
}

fn run_with_phases(program: &[i64], phases: &[i64]) -> i64 {
    let mut input = 0;
    for &phase in phases {
        let (io, tx, rx) = AsyncIO::new();
        let _ = tx.send(phase);
        let _ = tx.send(input);

        let mut computer = IntCodeComputer::new(program.to_owned(), io);
        computer.run();
        drop(computer);

        input = from_fn(|| rx.recv().ok()).last().unwrap();
    }
    input
}

fn run_with_phases_async(program: &[i64], phases: &[i64]) -> i64 {
    // setup io
    let (a_io, a_tx, a_rx) = intcode::AsyncIO::new();
    let (b_io, b_tx, b_rx) = intcode::AsyncIO::new();
    let (c_io, c_tx, c_rx) = intcode::AsyncIO::new();
    let (d_io, d_tx, d_rx) = intcode::AsyncIO::new();
    let (e_io, e_tx, e_rx) = intcode::AsyncIO::new();
    let (o_tx, o_rx) = channel();

    let _ = a_tx.send(phases[0]);
    let _ = a_tx.send(0);
    let _ = b_tx.send(phases[1]);
    let _ = c_tx.send(phases[2]);
    let _ = d_tx.send(phases[3]);
    let _ = e_tx.send(phases[4]);

    let ab_cnx = Connector::new(b_tx, a_rx);
    let bc_cnx = Connector::new(c_tx, b_rx);
    let cd_cnx = Connector::new(d_tx, c_rx);
    let de_cnx = Connector::new(e_tx, d_rx);
    let eao_cnx = Connector::multiplexed(vec![a_tx, o_tx], e_rx);

    // setup computers
    let mut a_computer = IntCodeComputer::new(program.to_vec(), a_io);
    let mut b_computer = IntCodeComputer::new(program.to_vec(), b_io);
    let mut c_computer = IntCodeComputer::new(program.to_vec(), c_io);
    let mut d_computer = IntCodeComputer::new(program.to_vec(), d_io);
    let mut e_computer = IntCodeComputer::new(program.to_vec(), e_io);

    // receive thread
    let output_thread = spawn(move || from_fn(|| o_rx.recv().ok()).last().unwrap());

    // run all in threads
    let threads = vec![
        spawn(move || a_computer.run()),
        spawn(move || b_computer.run()),
        spawn(move || c_computer.run()),
        spawn(move || d_computer.run()),
        spawn(move || e_computer.run()),
        spawn(move || ab_cnx.run()),
        spawn(move || bc_cnx.run()),
        spawn(move || cd_cnx.run()),
        spawn(move || de_cnx.run()),
        spawn(move || eao_cnx.run()),
    ];

    // wait
    for t in threads {
        let _ = t.join();
    }

    // wait for final output value
    output_thread.join().unwrap()
}
