use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn read_input() -> Result<Vec<usize>> {
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map_err(Into::into)
                .and_then(|line| line.parse().map_err(Into::into))
        })
        .collect()
}

fn part1(input: &[usize]) -> usize {
    for i in 25..(input.len()) {
        let sums = sums(&input[i - 25..i]);
        if !sums.contains(&input[i]) {
            return input[i];
        }
    }

    unreachable!()
}

fn sums(input: &[usize]) -> HashSet<usize> {
    let mut sums = HashSet::new();
    for (i, x) in input.iter().enumerate() {
        for (j, y) in input.iter().enumerate() {
            if i == j {
                continue;
            }
            sums.insert(x + y);
        }
    }
    sums
}

fn part2(input: &[usize]) -> usize {
    let invalid = part1(input);
    let mut parts = VecDeque::new();
    let mut sum = 0;

    'outer: for val in input {
        if sum == invalid {
            break;
        }
        while sum + val > invalid {
            if sum == 0 {
                continue 'outer;
            }

            sum -= parts.pop_front().unwrap();
        }

        sum += val;
        parts.push_back(val);
    }

    if sum == invalid {
        **parts.iter().min().unwrap() + **parts.iter().max().unwrap()
    } else {
        unreachable!()
    }
}
