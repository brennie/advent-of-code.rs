use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn read_input() -> Result<Vec<usize>> {
    BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|r| {
            r.map_err(Into::into)
                .and_then(|line| line.parse().map_err(Into::into))
        })
        .collect()
}

fn main() -> Result<()> {
    let joltages = read_input()?;

    println!("part 1: {}", part1(&joltages));
    println!("part 2: {}", part2(&joltages));

    Ok(())
}

fn part1(joltages: &[usize]) -> usize {
    let joltages = {
        let mut joltages = Vec::from(joltages);
        joltages.push(0);
        joltages.push(*joltages.iter().max().unwrap() + 3);
        joltages.sort();
        joltages
    };

    let mut diff_1 = 0;
    let mut diff_3 = 0;

    for i in 1..joltages.len() {
        match joltages[i] - joltages[i - 1] {
            1 => diff_1 += 1,
            2 => continue,
            3 => diff_3 += 1,
            _ => unreachable!(),
        }
    }
    diff_1 * diff_3
}

fn part2(joltages: &[usize]) -> usize {
    let max_joltage = *joltages.iter().max().unwrap() + 3;
    let joltages = {
        let mut joltages: HashSet<_> = joltages.iter().cloned().collect();
        joltages.insert(max_joltage);
        joltages.insert(0);
        joltages
    };

    let mut open = VecDeque::<usize>::new();
    let mut closed = HashSet::new();
    open.push_back(*joltages.iter().max().unwrap());

    let mut paths = HashMap::<usize, usize>::new();

    while !open.is_empty() {
        // Pick the next highest joltage `j`.
        let j = open.pop_front().unwrap();

        if closed.contains(&j) {
            continue;
        }
        closed.insert(j);

        // There are `j_count` paths from joltage `j` to `max_joltage`.
        // If `j == max_joltage` then there is exactly one path: itself.
        let j_count = *paths.entry(j).or_insert(1);

        // There are at most 3 adaptors that could connect to `j`: `j - 1`,
        // `j - 2`, and `j - 3`.
        for i in 1..=3 {
            if j >= i {
                let prev = j - i;
                if joltages.contains(&prev) {
                    *paths.entry(prev).or_insert(0) += j_count;
                    open.push_back(prev);
                }
            }
        }
    }

    *paths.get(&0).unwrap()
}
