use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str;

fn read_input() -> Result<Vec<usize>, Box<dyn Error>> {
    let mut buf = String::new();
    File::open("input.txt")?.read_to_string(&mut buf)?;
    buf[..buf.len() - 1]
        .split(',')
        .map(|s| str::parse::<usize>(&s).map_err(Into::into))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    {
        let mut mem = mem.clone();

        mem[1] = 12;
        mem[2] = 2;

        process_opcodes(&mut mem);

        println!("part 1: {}", mem[0]);
    }

    for i in 0..100 {
        for j in 0..100 {
            let mut mem = mem.clone();
            mem[1] = i;
            mem[2] = j;

            process_opcodes(&mut mem);

            if mem[0] == 19690720 {
                println!("part 2: {}", 100 * mem[1] + mem[2]);
                return Ok(())
            }
        }
    }

    // mem[1] = 12;
    // mem[2] = 2;

    Ok(())
}

fn process_opcodes(mem: &mut [usize]) {
    for i in (0..mem.len()).step_by(4) {
        let opcode = mem[i];
        if opcode == 99 {
            break;
        }

        let addr1 = mem[i + 1];
        let val1 = mem[addr1];
        let addr2 = mem[i + 2];
        let val2 = mem[addr2];

        let result_addr = mem[i + 3];

        match opcode {
            1 => mem[result_addr] = val1 + val2,
            2 => mem[result_addr] = val1 * val2,
            _ => panic!("invalid opcode"),
        }
    }
}
