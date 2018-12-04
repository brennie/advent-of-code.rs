use std::collections::HashSet;
use std::process::exit;

use day01::{read_offsets, Result};

fn run() -> Result<i32> {
    let offsets = read_offsets()?;

    let mut freq = 0;
    let mut freq_counts = HashSet::new();

    for offset in offsets.into_iter().cycle() {
        freq += offset;

        if !freq_counts.insert(freq) {
            // HashSet.insert(k) returns false when the k is already present.
            return Ok(freq);
        }
    }

    unreachable!()
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
