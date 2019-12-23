use std::collections::VecDeque;
use std::iter::repeat;
use std::ops::{Index, IndexMut};

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

pub struct Vm {
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

    pub fn run(&mut self) -> RunResult {
        loop {
            let instr = Instr::parse_from(&self.mem.as_ref()[self.pc..]);
            let len = instr.len();
            self.pc += len;
            match self.exec(instr) {
                Some(ExecResult::Halt) => return RunResult::Halt,
                Some(ExecResult::InputRequired) => {
                    self.pc -= len;
                    return RunResult::InputRequired;
                }
                Some(ExecResult::Output(o)) => {
                    return RunResult::Output(o);
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
                match self.input.pop_front() {
                    Some(v) => {
                        self.mem[addr] = v;
                        None
                    }
                    None => Some(ExecResult::InputRequired),
                }
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
    InputRequired,
}

#[derive(Clone, Copy, Debug)]
pub enum RunResult {
    Halt,
    Output(isize),
    InputRequired,
}

impl RunResult {
    pub fn as_output(self) -> Option<isize> {
        match self {
            RunResult::Output(v) => Some(v),
            _ => None,
        }
    }
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
