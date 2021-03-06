use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::sync::mpsc::{channel, Receiver, Sender};

pub trait IO {
    fn get(&mut self) -> io::Result<i64>;
    fn put(&mut self, val: i64) -> io::Result<()>;
}

pub struct NoIO {}

impl IO for NoIO {
    fn get(&mut self) -> io::Result<i64> {
        Ok(0)
    }

    fn put(&mut self, _: i64) -> io::Result<()> {
        Ok(())
    }
}

pub struct AsyncIO {
    tx: Sender<i64>,
    rx: Receiver<i64>,
}

impl AsyncIO {
    pub fn new() -> (Self, Sender<i64>, Receiver<i64>) {
        let (itx, orx) = channel();
        let (otx, irx) = channel();
        let s = Self { tx: itx, rx: irx };
        (s, otx, orx)
    }
}

impl IO for AsyncIO {
    fn get(&mut self) -> io::Result<i64> {
        self.rx
            .recv()
            .map_err(|e| io::Error::new(ErrorKind::BrokenPipe, e))
    }

    fn put(&mut self, val: i64) -> io::Result<()> {
        self.tx
            .send(val)
            .map_err(|e| io::Error::new(ErrorKind::BrokenPipe, e))
    }
}

pub struct Connector {
    tx: Vec<Sender<i64>>,
    rx: Receiver<i64>,
}

impl Connector {
    pub fn new(tx: Sender<i64>, rx: Receiver<i64>) -> Self {
        Self { tx: vec![tx], rx }
    }

    pub fn multiplexed(tx: Vec<Sender<i64>>, rx: Receiver<i64>) -> Self {
        Self { tx, rx }
    }

    pub fn run(&self) {
        while let Ok(data) = self.rx.recv() {
            for tx in self.tx.iter() {
                let _ = tx.send(data);
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(n: i64) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("invalid parameter mode"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Opcode {
    Add,
    Mul,
    In,
    Out,
    Jit,
    Jif,
    Lt,
    Eq,
    Arb,
    Halt,
}

impl From<i64> for Opcode {
    fn from(n: i64) -> Self {
        match n {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::In,
            4 => Opcode::Out,
            5 => Opcode::Jit,
            6 => Opcode::Jif,
            7 => Opcode::Lt,
            8 => Opcode::Eq,
            9 => Opcode::Arb,
            99 => Opcode::Halt,
            _ => panic!("Invalid opcode"),
        }
    }
}

fn decode_instruction(instruction: i64) -> (Opcode, Vec<ParameterMode>) {
    let opcode = instruction % 100;
    let param_modes = vec![
        ((instruction / 100) % 10).into(),
        ((instruction / 1000) % 10).into(),
        ((instruction / 10000) % 10).into(),
    ];

    (opcode.into(), param_modes)
}

pub fn parse_program<R: Read>(r: R) -> Vec<i64> {
    BufReader::new(r)
        .split(b',')
        .flatten()
        .flat_map(String::from_utf8)
        .flat_map(|s| s.parse())
        .collect()
}

pub struct IntCodeComputer<T>
where
    T: IO,
{
    pub program: Vec<i64>,
    pub io: T,
    ip: usize,
    relative_base: i64,
    halt: bool,
}

impl<T> IntCodeComputer<T>
where
    T: IO,
{
    pub fn new(program: Vec<i64>, io: T) -> Self {
        Self {
            ip: 0,
            program,
            io,
            relative_base: 0,
            halt: false,
        }
    }

    pub fn run(&mut self) {
        while !self.halt {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let (opcode, pms) = decode_instruction(self.read_memory(self.ip));
        match opcode {
            Opcode::Add => self.add(&pms),
            Opcode::Mul => self.mul(&pms),
            Opcode::In => self.input(&pms),
            Opcode::Out => self.output(&pms),
            Opcode::Jit => self.jump_if_true(&pms),
            Opcode::Jif => self.jump_if_false(&pms),
            Opcode::Lt => self.less_than(&pms),
            Opcode::Eq => self.equals(&pms),
            Opcode::Arb => self.adjust_relative_base(&pms),
            Opcode::Halt => self.halt(),
        }
    }

    fn write_memory(&mut self, idx: i64, val: i64) {
        if idx < 0 {
            panic!("negative index");
        }
        let idx = idx as usize;
        if idx >= self.program.len() {
            self.program.resize(idx + 1, 0);
        }
        self.program[idx] = val;
    }

    fn read_memory(&self, idx: usize) -> i64 {
        if idx >= self.program.len() {
            0
        } else {
            self.program[idx]
        }
    }

    fn parameter(&self, idx: usize, parameter_modes: &[ParameterMode]) -> i64 {
        let param = self.read_memory(self.ip + idx + 1);
        match parameter_modes[idx] {
            ParameterMode::Position => self.read_memory(param as usize),
            ParameterMode::Immediate => param,
            ParameterMode::Relative => self.read_memory((param + self.relative_base) as usize),
        }
    }

    fn dest(&self, idx: usize, parameter_modes: &[ParameterMode]) -> i64 {
        let dest = self.read_memory(self.ip + idx + 1);
        match parameter_modes[idx] {
            ParameterMode::Position => dest,
            ParameterMode::Immediate => panic!("invalid mode for dest"),
            ParameterMode::Relative => dest + self.relative_base,
        }
    }

    fn operands3(&self, parameter_modes: &[ParameterMode]) -> (i64, i64, i64) {
        let param1 = self.parameter(0, parameter_modes);
        let param2 = self.parameter(1, parameter_modes);
        let dest = self.dest(2, parameter_modes);
        (param1, param2, dest)
    }

    fn operands2(&self, parameter_modes: &[ParameterMode]) -> (i64, i64) {
        let param1 = self.parameter(0, parameter_modes);
        let param2 = self.parameter(1, parameter_modes);
        (param1, param2)
    }

    fn add(&mut self, parameter_modes: &[ParameterMode]) {
        let (o1, o2, d) = self.operands3(parameter_modes);
        self.write_memory(d, o1 + o2);
        self.ip += 4;
    }

    fn mul(&mut self, parameter_modes: &[ParameterMode]) {
        let (o1, o2, d) = self.operands3(parameter_modes);
        self.write_memory(d, o1 * o2);
        self.ip += 4;
    }

    fn input(&mut self, parameter_modes: &[ParameterMode]) {
        let dest = self.dest(0, parameter_modes);
        if let Ok(value) = self.io.get() {
            self.write_memory(dest, value);
            self.ip += 2;
        } else {
            self.halt = true;
        }
    }

    fn output(&mut self, parameter_modes: &[ParameterMode]) {
        let param = self.parameter(0, parameter_modes);
        if self.io.put(param).is_ok() {
            self.ip += 2;
        } else {
            self.halt = true;
        }
    }

    fn jump_if_true(&mut self, parameter_modes: &[ParameterMode]) {
        let (o1, o2) = self.operands2(parameter_modes);
        self.ip = if o1 != 0 { o2 as usize } else { self.ip + 3 };
    }

    fn jump_if_false(&mut self, parameter_modes: &[ParameterMode]) {
        let (o1, o2) = self.operands2(parameter_modes);
        self.ip = if o1 == 0 { o2 as usize } else { self.ip + 3 };
    }

    fn less_than(&mut self, parameter_modes: &[ParameterMode]) {
        let (o1, o2, d) = self.operands3(parameter_modes);
        self.write_memory(d, if o1 < o2 { 1 } else { 0 });
        self.ip += 4;
    }

    fn equals(&mut self, parameter_modes: &[ParameterMode]) {
        let (o1, o2, d) = self.operands3(parameter_modes);
        self.write_memory(d, if o1 == o2 { 1 } else { 0 });
        self.ip += 4;
    }

    fn adjust_relative_base(&mut self, parameter_modes: &[ParameterMode]) {
        let param = self.parameter(0, parameter_modes);
        self.relative_base += param;
        self.ip += 2;
    }

    fn halt(&mut self) {
        self.halt = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_instruction() {
        assert_eq!(
            decode_instruction(1002),
            (
                Opcode::Mul,
                vec![
                    ParameterMode::Position,
                    ParameterMode::Immediate,
                    ParameterMode::Position
                ]
            )
        );
    }

    fn assert_program_output_eq(program: Vec<i64>, output: Vec<i64>) {
        let mut computer = IntCodeComputer::new(program, NoIO {});

        computer.run();
        assert_eq!(computer.program, output);
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
        assert_program_output_eq(vec![1101, 100, -1, 4, 0], vec![1101, 100, -1, 4, 99]);
    }
}
