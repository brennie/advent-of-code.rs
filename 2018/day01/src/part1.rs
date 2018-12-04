use std::ops::Add;
use std::process::exit;

use day01::{read_offsets, Result};

fn run() -> Result<i32> {
    let freq = read_offsets()?.into_iter().fold(0, Add::add);
    Ok(freq)
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(result) => println!("{}", result),
    }
}
