use crate::{
    intcode::{parse_program, IntCodeComputer, IO},
    solver::Solver,
};
use itertools::Itertools;
use std::iter::from_fn;
use std::{
    io::Read,
    sync::mpsc::{channel, Receiver, Sender},
    thread::spawn,
};

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

fn run_with_phases(program: &Vec<i64>, phases: &[i64]) -> i64 {
    let mut input = 0;
    for &phase in phases {
        let mut computer = IntCodeComputer::new(program.clone(), VecIO::new(phase, input));
        computer.run();
        input = computer.io.output;
    }
    input
}

fn run_with_phases_async(program: &Vec<i64>, phases: &[i64]) -> i64 {
    // setup io
    let mut a_io = AsyncIO::new_with_init(phases[0], 0);
    let mut b_io = AsyncIO::new(phases[1]);
    let mut c_io = AsyncIO::new(phases[2]);
    let mut d_io = AsyncIO::new(phases[3]);
    let mut e_io = AsyncIO::new(phases[4]);

    a_io.connect_receiver(e_io.get_receiver());
    b_io.connect_receiver(a_io.get_receiver());
    c_io.connect_receiver(b_io.get_receiver());
    d_io.connect_receiver(c_io.get_receiver());
    e_io.connect_receiver(d_io.get_receiver());
    let output_receiver = e_io.get_receiver();

    // setup computers
    let mut a_computer = IntCodeComputer::new(program.clone(), a_io);
    let mut b_computer = IntCodeComputer::new(program.clone(), b_io);
    let mut c_computer = IntCodeComputer::new(program.clone(), c_io);
    let mut d_computer = IntCodeComputer::new(program.clone(), d_io);
    let mut e_computer = IntCodeComputer::new(program.clone(), e_io);

    // receive thread
    let output_thread = spawn(move || from_fn(|| output_receiver.recv().ok()).last().unwrap());

    // run all in threads
    let threads = vec![
        spawn(move || a_computer.run()),
        spawn(move || b_computer.run()),
        spawn(move || c_computer.run()),
        spawn(move || d_computer.run()),
        spawn(move || e_computer.run()),
    ];

    // wait
    for t in threads {
        let _ = t.join();
    }

    // wait for final output value
    output_thread.join().unwrap()
}

struct VecIO {
    input: Vec<i64>,
    output: i64,
}

impl IO for VecIO {
    fn get(&mut self) -> i64 {
        self.input.pop().unwrap()
    }

    fn put(&mut self, val: i64) {
        self.output = val;
    }
}

impl VecIO {
    fn new(phase: i64, input: i64) -> Self {
        Self {
            input: vec![input, phase],
            output: 0,
        }
    }
}

struct AsyncIO {
    tx: Vec<Sender<i64>>,
    rx: Option<Receiver<i64>>,
    buffer: Vec<i64>,
}

impl AsyncIO {
    fn new_with_init(phase: i64, init: i64) -> Self {
        Self {
            tx: vec![],
            rx: None,
            buffer: vec![init, phase],
        }
    }

    fn new(phase: i64) -> Self {
        Self {
            tx: vec![],
            rx: None,
            buffer: vec![phase],
        }
    }

    fn get_receiver(&mut self) -> Receiver<i64> {
        let (tx, rx) = channel();
        self.tx.push(tx);
        rx
    }

    fn connect_receiver(&mut self, rx: Receiver<i64>) {
        self.rx = Some(rx);
    }
}

impl IO for AsyncIO {
    fn get(&mut self) -> i64 {
        if let Some(val) = self.buffer.pop() {
            val
        } else if let Some(rx) = &self.rx {
            rx.recv().unwrap()
        } else {
            0
        }
    }

    fn put(&mut self, val: i64) {
        for tx in &self.tx {
            let _ = tx.send(val);
        }
    }
}
