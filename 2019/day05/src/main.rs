use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

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

    println!("part 1: ");
    {
        let mut mem = mem.clone();
        run(&mut mem, 1);
    }

    println!("part 2: ");
    {
        let mut mem = mem.clone();
        run(&mut mem, 5);
    }
    Ok(())
}

fn run(mem: &mut [isize], input: isize) {
    let mut pc = 0usize;

    loop {
        let instr = parse_instr(&mem[pc..]);
        match instr.execute(mem, input) {
            Some(ExecResult::Halt) => break,
            Some(ExecResult::Output(output)) => {
                println!("output: {}", output);
                pc += instr.len();
            }
            Some(ExecResult::Jump(addr)) => pc = addr,
            None => pc += instr.len(),
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

    fn execute(&self, mem: &mut [isize], input: isize) -> Option<ExecResult> {
        match self {
            Instr::Add(v1, v2, m) => {
                let op1 = v1.fetch(&mem);
                let op2 = v2.fetch(&mem);

                mem[*m] = op1 + op2;

                None
            }

            Instr::Mul(v1, v2, m) => {
                let op1 = v1.fetch(&mem);
                let op2 = v2.fetch(&mem);

                mem[*m] = op1 * op2;

                None
            }

            Instr::Input(m) => {
                mem[*m] = input;

                None
            }

            Instr::Output(v) => {
                Some(ExecResult::Output(v.fetch(&mem)))
            }

            Instr::JumpIfTrue(v1, v2) => {
                if v1.fetch(&mem) != 0 {
                    Some(ExecResult::Jump(v2.fetch(&mem) as usize))
                } else {
                    None
                }
            }

            Instr::JumpIfFalse(v1, v2) => {
                if v1.fetch(&mem) == 0 {
                    Some(ExecResult::Jump(v2.fetch(&mem) as usize))
                } else {
                    None
                }
            }

            Instr::LessThan(v1, v2, m) => {
                let op1 = v1.fetch(&mem);
                let op2 = v2.fetch(&mem);

                mem[*m] = if op1 < op2 { 1 } else { 0 };
                None
            }

            Instr::Equals(v1, v2, m) => {
                let op1 = v1.fetch(&mem);
                let op2 = v2.fetch(&mem);

                mem[*m] = if op1 == op2 { 1 } else { 0 };
                None
            }

            Instr::Halt => Some(ExecResult::Halt),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Immediate(isize),
    Position(usize),
}

impl Value {
    fn fetch(&self, mem: &[isize]) -> isize {
        match self {
            Value::Immediate(v) => *v,
            Value::Position(a) => mem[*a],
        }
    }
}

pub fn parse_instr(mem: &[isize]) -> Instr {
    let instr = mem[0];

    let opcode = instr % 100;
    let mode1 = (instr / 100) % 10;
    let mode2 = (instr / 1000) % 10;
    let mode3 = (instr / 10000) % 10;

    match opcode {
        1 => {
            let v1 = if mode1 == 1 { Value::Immediate(mem[1]) } else { Value::Position(mem[1] as usize) };
            let v2 = if mode2 == 1 { Value::Immediate(mem[2]) } else { Value::Position(mem[2] as usize) };

            Instr::Add(
                v1,
                v2,
                mem[3] as usize,
            )
        },
        2 => {
            let v1 = if mode1 == 1 { Value::Immediate(mem[1]) } else { Value::Position(mem[1] as usize) };
            let v2 = if mode2 == 1 { Value::Immediate(mem[2]) } else { Value::Position(mem[2] as usize) };

            Instr::Mul(
                v1,
                v2,
                mem[3] as usize,
            )
        },
        3 => Instr::Input(mem[1] as usize),
        4 => {
            let v1 = if mode1 == 1 { Value::Immediate(mem[1]) } else { Value::Position(mem[1] as usize) };
            Instr::Output(v1)
        }
        5 => {
            let v1 = if mode1 == 1 { Value::Immediate(mem[1]) } else { Value::Position(mem[1] as usize) };
            let v2 = if mode2 == 1 { Value::Immediate(mem[2]) } else { Value::Position(mem[2] as usize) };

            Instr::JumpIfTrue(
                v1,
                v2,
            )
        }
        6 =>{
            let v1 = if mode1 == 1 { Value::Immediate(mem[1]) } else { Value::Position(mem[1] as usize) };
            let v2 = if mode2 == 1 { Value::Immediate(mem[2]) } else { Value::Position(mem[2] as usize) };
             Instr::JumpIfFalse(v1, v2)
        }
        7 => {
            let v1 = if mode1 == 1 { Value::Immediate(mem[1]) } else { Value::Position(mem[1] as usize) };
            let v2 = if mode2 == 1 { Value::Immediate(mem[2]) } else { Value::Position(mem[2] as usize) };
            // let v3 = if mode3 == 1 { Value::Immediate(mem[3]) } else { Value::Position(mem[3] as usize) };

            Instr::LessThan(v1, v2, mem[3] as usize)
        }
        8 => {
            let v1 = if mode1 == 1 { Value::Immediate(mem[1]) } else { Value::Position(mem[1] as usize) };
            let v2 = if mode2 == 1 { Value::Immediate(mem[2]) } else { Value::Position(mem[2] as usize) };
            // let v3 = if mode3 == 1 { Value::Immediate(mem[3]) } else { Value::Position(mem[3] as usize) };

            Instr::Equals(v1, v2, mem[3] as usize)
        }

        99 => Instr::Halt,
        v => {
            eprintln!("{:?}", v);
            unimplemented!();
        }
    }
}
