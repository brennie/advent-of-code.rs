use std::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::prelude::*;
use std::fs::File;

use indoc::indoc;

use crate::intcode::Vm;

mod intcode;

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    {

        let program = indoc!(
            r#"NOT C J
            AND D J
            NOT A T
            OR T J
            WALK
            "#);

        let input = program.as_bytes().iter().map(|&c| c as isize).collect::<Vec<_>>();

        let mut vm = Vm::new_with_input(&mem, &input);

        while let Some(c) = vm.run() {
            if 0 < c && c <= 128 {
                print!("{}", c as u8 as char);
            } else {
                println!("part 1: {}", c);
            }
        }
    }
    {
        let program = indoc!(
            r#"NOT A J
            NOT C T
            OR T J
            NOT B T
            OR T J
            AND D J
            NOT E T
            NOT T T
            OR H T
            AND T J
            RUN
            "#);

        let input = program.as_bytes().iter().map(|&c| c as isize).collect::<Vec<_>>();

        let mut vm = Vm::new_with_input(&mem, &input);

        while let Some(c) = vm.run() {
            if 0 < c && c <= 128 {
                print!("{}", c as u8 as char);
            } else {
                println!("part 2: {}", c);
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
