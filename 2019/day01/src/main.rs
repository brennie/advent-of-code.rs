use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::str;

fn read_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let reader = BufReader::new(File::open("input.txt")?);

    reader
        .lines()
        .map(|r| r
            .map_err(Box::<dyn Error>::from)
            .and_then(|s| str::parse::<i32>(&s).map_err(Into::into)
            ))
        .collect()
}

fn calculate_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + calculate_fuel(fuel)
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let modules = read_input()?;
    let fuel: i32 = modules.iter()
        .map(|i| i / 3 - 2)
        .sum();

    println!("part 1: {}", fuel);

    let real_fuel: i32 = modules.iter()
        .map(|&i| calculate_fuel(i))
        .sum();

    println!("part 2: {}", real_fuel);

    Ok(())
}
