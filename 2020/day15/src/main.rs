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
    // Keep track of terms said previously, *except* when that term is the most
    // recent. Since we always know the most recent term is at index `i - 1`,
    // we only need to store a single entry for each number to know when it was
    // previously said.
    //
    // If we stored the *all* entries here (including `most_recent =
    // input[input.len() - 1]`) and `most_recent` appeared previously in the
    // input (i.e., before `input.len() - 1`) we would get an incorrect result
    // due to not knowing the previous time that it was said.
    let mut last_spoken = input
        .iter()
        .take(input.len() - 1)
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<HashMap<usize, usize>>();

    let mut most_recent = input[input.len() - 1];
    for i in input.len()..target {
        most_recent = match last_spoken.entry(most_recent) {
            Entry::Occupied(mut e) => {
                let next = i - 1 - e.get();
                *e.get_mut() = i - 1;

                next
            }

            Entry::Vacant(e) => {
                e.insert(i - 1);
                0
            }
        };
    }
    most_recent
}
