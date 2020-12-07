use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let input = read_input()?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn read_input() -> Result<Vec<Vec<HashSet<char>>>> {
    let mut f = BufReader::new(File::open("input")?);

    let mut group = Vec::new();
    let mut groups = Vec::new();
    for line in f.lines() {
        let line = line?;

        if line.len() == 0 {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line.chars().collect());
        }
    }

    if group.len() > 0 {
        groups.push(group);
    }

    Ok(groups)
}

fn part1(groups: &[Vec<HashSet<char>>]) -> usize {
    groups
        .iter()
        .map(|v: &Vec<HashSet<char>>| {
            let mut q = HashSet::<char>::new();

            for u in v {
                q.extend(u.iter());
            }

            q.len()
        })
        .sum::<usize>()
}

fn part2(groups: &[Vec<HashSet<char>>]) -> usize {
    groups
        .iter()
        .map(|v: &Vec<HashSet<char>>| {
            let mut q = HashMap::<char, usize>::new();

            for u in v {
                for c in u.iter() {
                    q.entry(*c).and_modify(|e| *e += 1).or_insert(1);
                }
            }

            q.iter()
                .map(|(c, n)| (*n == v.len()) as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
}
