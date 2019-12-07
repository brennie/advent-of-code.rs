use std::collections::VecDeque;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

fn read_input() -> Result<Vec<isize>, Box<dyn Error>> {
    let mut buf = String::new();
    File::open("input")?.read_to_string(&mut buf)?;
    buf[..buf.len() - 1]
        .split(',')
        .map(|s| str::parse::<isize>(&s).map_err(Into::into))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    {
        let phases = [0, 1, 2, 3, 4];
        let mut max: Option<isize> = None;

        for phase in phases.iter().permutations(5) {
            let o1 = Vm::new_with_input(&mem, &[*phase[0], 0]).run().unwrap();
            let o2 = Vm::new_with_input(&mem, &[*phase[1], o1]).run().unwrap();
            let o3 = Vm::new_with_input(&mem, &[*phase[2], o2]).run().unwrap();
            let o4 = Vm::new_with_input(&mem, &[*phase[3], o3]).run().unwrap();
            let o5 = Vm::new_with_input(&mem, &[*phase[4], o4]).run().unwrap();

            if let Some(prev_max) = max {
                max = Some(std::cmp::max(prev_max, o5));
            } else {
                max = Some(o5);
            }
        }

        println!("part 1: {:?}", max);
    }

    {
        let phases = vec![5, 6, 7, 8, 9];
        let mut max: Option<isize> = None;

        for phase in phases.iter().permutations(5) {
            let mut vm1 = Vm::new_with_input(&mem, &[*phase[0]]);
            let mut vm2 = Vm::new_with_input(&mem, &[*phase[1]]);
            let mut vm3 = Vm::new_with_input(&mem, &[*phase[2]]);
            let mut vm4 = Vm::new_with_input(&mem, &[*phase[3]]);
            let mut vm5 = Vm::new_with_input(&mem, &[*phase[4]]);

            let o1 = vm1.with_input(0).run().unwrap();
            let o2 = vm2.with_input(o1).run().unwrap();
            let o3 = vm3.with_input(o2).run().unwrap();
            let o4 = vm4.with_input(o3).run().unwrap();
            let mut output = vm5.with_input(o4).run().unwrap();

            loop {
                let o1 = match vm1.with_input(output).run() {
                    Some(o) => o,
                    None => break,
                };

                let o2 = match vm2.with_input(o1).run() {
                    Some(o) => o,
                    None => break,
                };

                let o3 = match vm3.with_input(o2).run() {
                    Some(o) => o,
                    None => break,
                };

                let o4 = match vm4.with_input(o3).run() {
                    Some(o) => o,
                    None => break,
                };

                output = match vm5.with_input(o4).run() {
                    Some(o) => o,
                    None => break,
                };
            }
            if let Some(prev_max) = max {
                max = Some(std::cmp::max(prev_max, output));
            } else {
                max = Some(output);
            }
        }

        println!("{:?}", max);
    }

    Ok(())
}

struct Vm {
    pc: usize,
    mem: Vec<isize>,
    input: VecDeque<isize>,
}

impl Vm {
    pub fn new(mem: &[isize]) -> Vm {
        Vm {
            pc: 0,
            mem: mem.into(),
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
            let instr = Instr::parse_from(&self.mem[self.pc..]);
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
            Instr::Add(v1, v2, m) => {
                let op1 = v1.fetch(&self.mem);
                let op2 = v2.fetch(&self.mem);

                self.mem[m] = op1 + op2;

                None
            }

            Instr::Mul(v1, v2, m) => {
                let op1 = v1.fetch(&self.mem);
                let op2 = v2.fetch(&self.mem);

                self.mem[m] = op1 * op2;

                None
            }

            Instr::Input(m) => {
                self.mem[m] = self.input.pop_front().unwrap();

                None
            }

            Instr::Output(v) => Some(ExecResult::Output(v.fetch(&self.mem))),

            Instr::JumpIfTrue(v1, v2) => {
                if v1.fetch(&self.mem) != 0 {
                    Some(ExecResult::Jump(v2.fetch(&self.mem) as usize))
                } else {
                    None
                }
            }

            Instr::JumpIfFalse(v1, v2) => {
                if v1.fetch(&self.mem) == 0 {
                    Some(ExecResult::Jump(v2.fetch(&self.mem) as usize))
                } else {
                    None
                }
            }

            Instr::LessThan(v1, v2, m) => {
                let op1 = v1.fetch(&self.mem);
                let op2 = v2.fetch(&self.mem);

                self.mem[m] = if op1 < op2 { 1 } else { 0 };
                None
            }

            Instr::Equals(v1, v2, m) => {
                let op1 = v1.fetch(&self.mem);
                let op2 = v2.fetch(&self.mem);

                self.mem[m] = if op1 == op2 { 1 } else { 0 };
                None
            }

            Instr::Halt => Some(ExecResult::Halt),
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    Add(Value, Value, usize),
    Mul(Value, Value, usize),
    Input(usize),
    Output(Value),
    JumpIfTrue(Value, Value),
    JumpIfFalse(Value, Value),
    LessThan(Value, Value, usize),
    Equals(Value, Value, usize),
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
        let _mode3 = (instr / 10000) % 10;

        match opcode {
            1 => Instr::Add(
                Value::new(mem[1], mode1),
                Value::new(mem[2], mode2),
                mem[3] as usize,
            ),
            2 => Instr::Mul(
                Value::new(mem[1], mode1),
                Value::new(mem[2], mode2),
                mem[3] as usize,
            ),
            3 => Instr::Input(mem[1] as usize),
            4 => Instr::Output(Value::new(mem[1], mode1)),
            5 => Instr::JumpIfTrue(Value::new(mem[1], mode1), Value::new(mem[2], mode2)),
            6 => Instr::JumpIfFalse(Value::new(mem[1], mode1), Value::new(mem[2], mode2)),
            7 => Instr::LessThan(
                Value::new(mem[1], mode1),
                Value::new(mem[2], mode2),
                mem[3] as usize,
            ),
            8 => Instr::Equals(
                Value::new(mem[1], mode1),
                Value::new(mem[2], mode2),
                mem[3] as usize,
            ),
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
            Instr::Halt => 1,
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Immediate(isize),
    Position(usize),
}

impl Value {
    fn new(value: isize, mode: isize) -> Value {
        match mode {
            0 => Value::Position(value as usize),
            1 => Value::Immediate(value),
            _ => unimplemented!("invalid value mode: {}", mode),
        }
    }

    fn fetch(&self, mem: &[isize]) -> isize {
        match self {
            Value::Immediate(v) => *v,
            Value::Position(a) => mem[*a],
        }
    }
}
