use std::collections::{HashSet, VecDeque};
use std::iter::repeat;
use std::ops::{Index, IndexMut};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use petgraph::algo::astar;
use petgraph::graphmap::GraphMap;

type G = GraphMap<Point, (), petgraph::Undirected>;

#[derive(Clone, Copy, Default, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn dist(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn direction(&self, other: &Point) -> isize {
        if other.y == self.y + 1 {
            1
        } else if other.y == self.y - 1 {
            2
        } else if other.x == self.x - 1 {
            3
        } else if other.x == self.x + 1 {
            4
        } else {
            unimplemented!()
        }
    }
}

fn neighbours(p: &Point, closed: &HashSet<Point>) -> Vec<Point> {
    let mut n = vec![];

    let q = Point { x: p.x, y: p.y + 1 };
    if !closed.contains(&q) {
        n.push(q);
    }
    let q = Point { x: p.x, y: p.y - 1 };
    if !closed.contains(&q) {
        n.push(q);
    }
    let q = Point { x: p.x + 1, y: p.y };
    if !closed.contains(&q) {
        n.push(q);
    }
    let q = Point { x: p.x - 1, y: p.y };
    if !closed.contains(&q) {
        n.push(q);
    }

    n
}

fn navigate(p: &mut Point, target: Point, g: &G, vm: &mut Vm) {
    if *p == target {
        return;
    }

    let (_, path) = astar(g, *p, |n| n == target, |_| 1, |p| p.dist(&target)).unwrap();

    for node in &path[1..] {
        let direction = p.direction(&node);
        let result = vm.with_input(direction).run().unwrap();

        assert_ne!(result, 0);

        *p = *node;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    let mut vm = Vm::new(&mem);
    let mut position = Point::default();
    let mut oxy_pos: Option<Point> = None;

    // The set of explored tiles.
    let mut closed = HashSet::new();
    closed.insert(position);

    // The set of unexplored tiles.
    let mut open = HashSet::new();
    open.extend(neighbours(&position, &closed));

    let mut g: G = G::new();

    while let Some(target) = open.iter().map(|p| *p).next() {
        open.remove(&target);

        let nearest = closed.iter().min_by_key(|p| p.dist(&target)).unwrap();

        navigate(&mut position, *nearest, &g, &mut vm);
        assert_eq!(position, *nearest);

        let direction = position.direction(&target);
        let result = vm.with_input(direction).run().unwrap();

        if result > 0 {
            closed.insert(target);
            open.extend(neighbours(&target, &closed));

            g.add_node(target);
            g.add_edge(position, target, ());

            position = target;

            if result == 2 {
                println!("found oxy");
                oxy_pos = Some(position);
            }
        }
    }

    let oxy_pos = oxy_pos.unwrap();
    let (len, _) = astar(
        &g,
        Point::default(),
        |n| n == oxy_pos,
        |_| 1,
        |p| p.dist(&oxy_pos),
    )
    .unwrap();
    println!("part 1: {}", len);

    {
        let mut empty = g.nodes().collect::<HashSet<_>>();
        let mut full = HashSet::<Point>::new();
        full.insert(oxy_pos);

        let mut t = 0;
        while empty.len() > 0 {
            let mut to_add = vec![];

            for full_node in &full {
                for n in g.neighbors(*full_node) {
                    empty.remove(&n);
                    to_add.push(n);
                }
            }

            t += 1;
            full.extend(&to_add);
            to_add.clear();
        }

        println!("part 2: {}", t);
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
