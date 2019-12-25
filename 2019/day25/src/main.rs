use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::intcode::{RunResult, Vm};

mod intcode;

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    {
        let mut vm = Vm::new(&mem);
        let stdin = io::stdin();

        loop {
            match vm.run() {
                RunResult::Halt => break,
                RunResult::InputRequired => {
                    print!("=> ");
                    io::stdout().flush()?;
                    let mut buf = String::new();
                    stdin.read_line(&mut buf)?;

                    for c in buf.chars() {
                        vm.with_input(c as u8 as isize);
                    }
                }
                RunResult::Output(c) => {
                    if 0 < c && c < 255 {
                        let c = c as u8 as char;
                        print!("{}", c);
                    } else {
                        panic!("non-ascii output")
                    }
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
