use std::collections::{HashMap, VecDeque};
use std::iter::repeat;
use std::ops::{Add, AddAssign};
use std::ops::{Index, IndexMut};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    println!("part 1: {}", paint(&mem, Colour::Black).len());

    println!("part 2:");
    print_hull(&paint(&mem, Colour::White));

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

fn paint(mem: &[isize], start_colour: Colour) -> HashMap<Point, Colour> {
    let mut vm = Vm::new(mem);
    let mut robot = Point::default();
    let mut direction = Point { x: 0, y: 1 };

    let mut panels = HashMap::new();
    panels.insert(robot, start_colour);

    loop {
        let panel = panels.entry(robot).or_default();
        let new_colour = match vm.with_input((*panel).into()).run() {
            Some(new_colour) => new_colour.into(),
            None => break,
        };

        *panel = new_colour;

        match vm.run().unwrap() {
            1 => direction = direction.rotate_right(),
            0 => direction = direction.rotate_left(),
            _ => unimplemented!(),
        }

        robot += direction;
    }

    panels
}

fn print_hull(panels: &HashMap<Point, Colour>) {
    // Find min x and y values so we can normalize the points to start at (0, 0).
    let min_x = panels.iter().map(|(p, _)| p.x).min().unwrap();
    let min_y = panels.iter().map(|(p, _)| p.y).min().unwrap();

    let width = (panels.iter().map(|(p, _)| p.x).max().unwrap() + 1 - min_x) as usize;
    let height = (panels.iter().map(|(p, _)| p.y).max().unwrap() + 1 - min_y) as usize;

    let mut points = vec![Colour::default(); width * height];

    for (Point { x, y }, colour) in panels {
        let x = (*x - min_x) as usize;
        let y = (*y - min_y) as usize;

        points[y * width + x] = *colour;
    }

    // The coordinate system has (0, 0) in the bottom left of the image.
    for y in (0..height).rev() {
        for x in 0..width {
            print!(
                "{}",
                match points[y * width + x] {
                    Colour::Black => ' ',
                    Colour::White => '#',
                }
            );
        }
        println!("");
    }
}

#[derive(Clone, Copy)]
enum Colour {
    Black,
    White,
}

impl Default for Colour {
    fn default() -> Self {
        Self::Black
    }
}

impl From<isize> for Colour {
    fn from(i: isize) -> Self {
        match i {
            0 => Colour::Black,
            1 => Colour::White,
            _ => unimplemented!(),
        }
    }
}

impl From<Colour> for isize {
    fn from(c: Colour) -> Self {
        match c {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    pub fn rotate_left(&self) -> Point {
        Point {
            x: self.y * -1,
            y: self.x,
        }
    }

    pub fn rotate_right(&self) -> Point {
        Point {
            x: self.y,
            y: self.x * -1,
        }
    }
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
