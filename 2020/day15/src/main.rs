use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = env::args()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow!("Usage: day15-2020 [input]"))?
        .split(",")
        .map(|s| s.parse().map_err(Into::into))
        .collect::<Result<Vec<usize>>>()?;

    println!("part 1: {}", run(&input, 2020));
    println!("part 2: {}", run(&input, 30000000));

    Ok(())
}

fn run(input: &[usize], target: usize) -> usize {
    let mut sequence = Vec::from(input);
    let mut last_spoken = sequence
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, vec![i]))
        .collect::<HashMap<usize, Vec<usize>>>();

    for i in sequence.len()..target {
        let most_recent = sequence[i - 1];

        let occurrences = last_spoken.get(&most_recent).unwrap();
        assert!(occurrences.len() >= 1);
        let next = if occurrences.len() == 1 {
            0
        } else {
            let j = occurrences[occurrences.len() - 1];
            let k = occurrences[occurrences.len() - 2];

            j - k
        };

        sequence.push(next);
        last_spoken
            .entry(next)
            .and_modify(|v| v.push(i))
            .or_insert_with(|| vec![i]);
    }

    sequence[target - 1]
}
