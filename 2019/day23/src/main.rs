use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use crate::intcode::{RunResult, Vm};

mod intcode;

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    {
        let mut vms = Vec::new();
        let mut input: Vec<VecDeque<(isize, isize)>> = Vec::new();

        for i in 0..50 {
            input.push(VecDeque::new());
            vms.push(Vm::new_with_input(&mem, &[i as isize]));
        }

        'outer: loop {
            for i in 0..50 {
                let vm = &mut vms[i];
                match vm.run() {
                    RunResult::Halt => panic!(),

                    RunResult::Output(addr) => {
                        let addr = addr as usize;

                        let x = vms[i].run().as_output().unwrap();
                        let y = vms[i].run().as_output().unwrap();

                        if addr == 255 {
                            println!("part 1: {}", y);
                            break 'outer;
                        }

                        vms[addr].with_input(x).with_input(y);
                    }

                    RunResult::InputRequired => {
                        vm.with_input(-1);
                    }
                }
            }
        }
    }

    {
        let mut vms = Vec::new();
        let mut input: Vec<VecDeque<(isize, isize)>> = Vec::new();
        let mut nat: Option<(isize, isize)> = None;

        for i in 0..50 {
            input.push(VecDeque::new());
            vms.push(Vm::new_with_input(&mem, &[i as isize]));
        }

        let mut seen = HashSet::new();

        loop {
            let mut idle_count = 0;

            for i in 0..50 {
                let vm = &mut vms[i];
                match vm.run() {
                    RunResult::Halt => panic!(),

                    RunResult::Output(addr) => {
                        let addr = addr as usize;

                        let x = vms[i].run().as_output().unwrap();
                        let y = vms[i].run().as_output().unwrap();

                        if addr == 255 {
                            nat = Some((x, y));
                        } else {
                            vms[addr].with_input(x).with_input(y);
                        }
                    }

                    RunResult::InputRequired => match vm.with_input(-1).run() {
                        RunResult::Halt => panic!(),
                        RunResult::Output(addr) => {
                            let addr = addr as usize;

                            let x = vms[i].run().as_output().unwrap();
                            let y = vms[i].run().as_output().unwrap();

                            if addr == 255 {
                                nat = Some((x, y));
                            } else {
                                vms[addr].with_input(x).with_input(y);
                            }
                        }
                        RunResult::InputRequired => {
                            idle_count += 1;
                        }
                    },
                }
            }

            if idle_count == 50 {
                let (x, y) = nat.unwrap();
                vms[0].with_input(x).with_input(y);

                if seen.contains(&y) {
                    println!("part 2: {}", y);
                    break;
                } else {
                    seen.insert(y);
                }
            }
        }
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
