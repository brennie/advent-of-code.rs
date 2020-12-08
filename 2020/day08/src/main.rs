use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let instr = read_input()?;
    println!("part 1: {}", part1(&instr));
    println!("part 2: {}", part2(&instr));

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    kind: InstructionKind,
    value: isize,
}

#[derive(Debug, Clone, Copy)]
enum InstructionKind {
    Acc,
    Jmp,
    Nop,
}

impl InstructionKind {
    fn from_str(s: &str) -> InstructionKind {
        match s {
            "acc" => InstructionKind::Acc,
            "jmp" => InstructionKind::Jmp,
            "nop" => InstructionKind::Nop,
            _ => panic!(),
        }
    }
}

struct Vm<'a> {
    memory: &'a [Instruction],
    pc: isize,
    acc: isize,
}

impl<'a> Vm<'a> {
    fn new(memory: &'a [Instruction]) -> Self {
        Vm {
            memory,
            pc: 0,
            acc: 0,
        }
    }

    fn pc(&self) -> usize {
        self.pc as usize
    }

    fn acc(&self) -> isize {
        return self.acc;
    }

    fn exec_one(&mut self) -> bool {
        if self.pc() == self.memory.len() {
            true
        } else {
            let instr = self.memory[self.pc()];
            match instr.kind {
                InstructionKind::Acc => {
                    self.acc += instr.value;
                    self.pc += 1;
                }
                InstructionKind::Jmp => {
                    self.pc += instr.value;
                }
                InstructionKind::Nop => {
                    self.pc += 1;
                }
            }

            false
        }
    }
}

fn read_input() -> Result<Vec<Instruction>> {
    let re = Regex::new(r#"^([a-z]{3}) ((?:\+|-)\d+)$"#).unwrap();
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map(|line| {
                let m = re.captures(&line).unwrap();

                Instruction {
                    kind: InstructionKind::from_str(&m[1]),
                    value: m[2].parse().unwrap(),
                }
            })
            .map_err(Into::into)
        })
        .collect()
}

fn run_until_loop(vm: &mut Vm) -> bool {
    let mut touched = HashSet::new();
    let mut pc = vm.pc();
    while !touched.contains(&pc) {
        touched.insert(pc);
        if vm.exec_one() {
            return false;
        }
        pc = vm.pc();
    }
    true
}

fn part1(instructions: &[Instruction]) -> isize {
    let mut vm = Vm::new(instructions);
    run_until_loop(&mut vm);

    return vm.acc();
}

fn part2(original: &[Instruction]) -> isize {
    let mut instructions = Vec::from(original);

    for i in 0..instructions.len() {
        let instr = original[i];

        match instr.kind {
            InstructionKind::Acc => continue,
            InstructionKind::Nop => instructions[i].kind = InstructionKind::Jmp,
            InstructionKind::Jmp => instructions[i].kind = InstructionKind::Nop,
        }

        let mut vm = Vm::new(&instructions);
        if !run_until_loop(&mut vm) {
            return vm.acc();
        }

        instructions[i].kind = instr.kind;
    }

    unreachable!()
}
