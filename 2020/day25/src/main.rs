use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

use itertools::Itertools;
fn main() -> Result<()> {
    let (pub1, pub2) = read_input()?;
    let mut loop_size = 0;
    let mut n = 1;
    loop {
        loop_size += 1;
        n = (n * 7) % 20201227;

        if n == pub1 {
            break;
        }
    }

    let mut n = 1;
    for _ in 0..loop_size {
        n = (n * pub2) % 20201227;
    }

    println!("encryption key: {}", n);
    Ok(())
}

fn read_input() -> Result<(u64, u64)> {
    let mut iter = BufReader::new(File::open("input")?)
        .lines()
        .map_results(|line| line.parse());

    let a = iter.next().unwrap().unwrap().unwrap();
    let b = iter.next().unwrap().unwrap().unwrap();

    Ok((a, b))
}
