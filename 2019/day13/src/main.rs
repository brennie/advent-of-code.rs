use std::collections::{HashSet, VecDeque};
use std::iter::repeat;
use std::ops::{Index, IndexMut};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Copy, Default, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Ball,
    Paddle,
}

impl Tile {
    fn new(t: isize) -> Tile {
        match t {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => unimplemented!(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    {
        let mut blocks = HashSet::<Point>::new();
        let mut vm = Vm::new(&mem);
        loop {
            let x = match vm.run() {
                Some(x) => x,
                None => break,
            };
            let y = vm.run().unwrap();
            let tile = Tile::new(vm.run().unwrap());

            if tile == Tile::Block {
                blocks.insert(Point { x, y });
            }
        }

        println!("part 1: {}", blocks.len());
    }

    {
        let mut mem = mem;
        mem[0] = 2;

        let mut vm = Vm::new(&mem);
        let mut score = None;

        let mut ball: Option<Point> = None;
        let mut paddle: Option<Point> = None;

        let score_pt = Point { x: -1, y: 0 };

        loop {
            let x = match vm.run() {
                Some(x) => x,
                None => break,
            };
            let y = vm.run().unwrap();
            let value = vm.run().unwrap();

            let p = Point { x, y };

            if p == score_pt {
                score = Some(value);
                continue;
            }

            let tile = Tile::new(value);

            match tile {
                Tile::Paddle => {
                    paddle = Some(p);
                }
                Tile::Ball => {
                    ball = Some(p);
                }
                _ => (),
            }

            match (ball, paddle) {
                (Some(ref b), Some(ref p)) => {
                    if p.x < b.x {
                        vm.with_input(1);
                        paddle = None;
                    } else if p.x > b.x {
                        vm.with_input(-1);
                        paddle = None;
                    } else {
                        vm.with_input(0);
                    }

                    ball = None;
                }
                _ => (),
            }
        }

        println!("part 2: {:?}", score);
    }

    Ok(())
}

fn read_input() -> Result<Vec<isize>, Box<dyn Error>> {
    let mut buf = String::new();
    File::open("input")?.read_to_string(&mut buf)?;
    buf[..buf.len() - 1]
        .split(',')
        .map(|s| str::parse::<isize>(&s).map_err(Into::into))
        .collect()
}
struct Mem {
    bytes: Vec<isize>,
    rel_base: isize,
}

impl AsRef<[isize]> for Mem {
    fn as_ref(&self) -> &[isize] {
        self.bytes.as_ref()
    }
}

impl Index<usize> for Mem {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.bytes.len() {
            &self.bytes[index]
        } else {
            &0
        }
    }
}

impl IndexMut<usize> for Mem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.bytes.len() {
            let to_extend = index - self.bytes.len() + 1;
            self.bytes.extend(repeat(0).take(to_extend));
        }

        &mut self.bytes[index]
    }
}

impl Mem {
    pub fn new(bytes: &[isize]) -> Mem {
        Mem {
            bytes: Vec::from(bytes),
            rel_base: 0,
        }
    }

    pub fn fetch(&self, value: Value) -> isize {
        if let Value::Immediate(v) = value {
            v
        } else {
            self[self.resolve(value)]
        }
    }

    pub fn adj_base(&mut self, offset: isize) {
        self.rel_base += offset;
    }

    fn resolve(&self, value: Value) -> usize {
        match value {
            Value::Position(addr) => addr,
            Value::Immediate(..) => unimplemented!("cannot resolve immediate value"),
            Value::Relative(offset) => (self.rel_base + offset) as usize,
        }
    }
}

struct Vm {
    pc: usize,
    mem: Mem,
    input: VecDeque<isize>,
}

impl Vm {
    pub fn new(mem: &[isize]) -> Vm {
        Vm {
            pc: 0,
            mem: Mem::new(mem),
            input: VecDeque::new(),
        }
    }

    pub fn new_with_input(mem: &[isize], input: &[isize]) -> Vm {
        let mut vm = Vm::new(mem);
        vm.input.extend(input);

        vm
    }

    pub fn with_input(&mut self, v: isize) -> &mut Self {
        self.input.push_back(v);
        self
    }

    pub fn run(&mut self) -> Option<isize> {
        loop {
            let instr = Instr::parse_from(&self.mem.as_ref()[self.pc..]);
            self.pc += instr.len();

            match self.exec(instr) {
                Some(ExecResult::Halt) => return None,
                Some(ExecResult::Output(o)) => {
                    return Some(o);
                }
                Some(ExecResult::Jump(addr)) => self.pc = addr,
                None => (),
            }
        }
    }

    fn exec(&mut self, instr: Instr) -> Option<ExecResult> {
        match instr {
            Instr::Add(v1, v2, v3) => {
                let op1 = self.mem.fetch(v1);
                let op2 = self.mem.fetch(v2);
                let addr = self.mem.resolve(v3);

                self.mem[addr] = op1 + op2;

                None
            }

            Instr::Mul(v1, v2, v3) => {
                let op1 = self.mem.fetch(v1);
                let op2 = self.mem.fetch(v2);
                let addr = self.mem.resolve(v3);

                self.mem[addr] = op1 * op2;

                None
            }

            Instr::Input(v) => {
                let addr = self.mem.resolve(v);
                self.mem[addr] = self.input.pop_front().unwrap();

                None
            }

            Instr::Output(v) => Some(ExecResult::Output(self.mem.fetch(v))),

            Instr::JumpIfTrue(v1, v2) => {
                if self.mem.fetch(v1) != 0 {
                    Some(ExecResult::Jump(self.mem.fetch(v2) as usize))
                } else {
                    None
                }
            }

            Instr::JumpIfFalse(v1, v2) => {
                if self.mem.fetch(v1) == 0 {
                    Some(ExecResult::Jump(self.mem.fetch(v2) as usize))
                } else {
                    None
                }
            }

            Instr::LessThan(v1, v2, v3) => {
                let op1 = self.mem.fetch(v1);
                let op2 = self.mem.fetch(v2);
                let addr = self.mem.resolve(v3);

                self.mem[addr] = if op1 < op2 { 1 } else { 0 };
                None
            }

            Instr::Equals(v1, v2, v3) => {
                let op1 = self.mem.fetch(v1);
                let op2 = self.mem.fetch(v2);
                let addr = self.mem.resolve(v3);

                self.mem[addr] = if op1 == op2 { 1 } else { 0 };
                None
            }

            Instr::AdjBase(v) => {
                let v = self.mem.fetch(v);
                self.mem.adj_base(v);
                None
            }

            Instr::Halt => Some(ExecResult::Halt),
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    Add(Value, Value, Value),
    Mul(Value, Value, Value),
    Input(Value),
    Output(Value),
    JumpIfTrue(Value, Value),
    JumpIfFalse(Value, Value),
    LessThan(Value, Value, Value),
    Equals(Value, Value, Value),
    AdjBase(Value),
    Halt,
}

enum ExecResult {
    Halt,
    Output(isize),
    Jump(usize),
}

impl Instr {
    pub fn parse_from(mem: &[isize]) -> Self {
        let instr = mem[0];

        let opcode = instr % 100;
        let mode1 = (instr / 100) % 10;
        let mode2 = (instr / 1000) % 10;
        let mode3 = (instr / 10000) % 10;

        match opcode {
            1 => Instr::Add(
                Value::new(mem[1], mode1),
                Value::new(mem[2], mode2),
                Value::new_output(mem[3], mode3),
            ),
            2 => Instr::Mul(
                Value::new(mem[1], mode1),
                Value::new(mem[2], mode2),
                Value::new_output(mem[3], mode3),
            ),
            3 => Instr::Input(Value::new_output(mem[1], mode1)),
            4 => Instr::Output(Value::new(mem[1], mode1)),
            5 => Instr::JumpIfTrue(Value::new(mem[1], mode1), Value::new(mem[2], mode2)),
            6 => Instr::JumpIfFalse(Value::new(mem[1], mode1), Value::new(mem[2], mode2)),
            7 => Instr::LessThan(
                Value::new(mem[1], mode1),
                Value::new(mem[2], mode2),
                Value::new_output(mem[3], mode3),
            ),
            8 => Instr::Equals(
                Value::new(mem[1], mode1),
                Value::new(mem[2], mode2),
                Value::new_output(mem[3], mode3),
            ),
            9 => Instr::AdjBase(Value::new(mem[1], mode1)),
            99 => Instr::Halt,
            v => {
                unimplemented!("unimplemented instruction {}", v);
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            Instr::Add(..) => 4,
            Instr::Mul(..) => 4,
            Instr::Input(..) => 2,
            Instr::Output(..) => 2,
            Instr::JumpIfTrue(..) => 3,
            Instr::JumpIfFalse(..) => 3,
            Instr::LessThan(..) => 4,
            Instr::Equals(..) => 4,
            Instr::AdjBase(..) => 2,
            Instr::Halt => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Immediate(isize),
    Position(usize),
    Relative(isize),
}

impl Value {
    fn new(value: isize, mode: isize) -> Value {
        match mode {
            0 => Value::Position(value as usize),
            1 => Value::Immediate(value),
            2 => Value::Relative(value),
            _ => unimplemented!("invalid value mode: {}", mode),
        }
    }

    fn new_output(value: isize, mode: isize) -> Value {
        let v = Value::new(value, mode);
        if let Value::Immediate(..) = v {
            panic!("Immediate mode unsupported as output");
        }
        v
    }
}
