use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = read_input()?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

struct Rule {
    i: usize,
    j: usize,
    letter: char,
}

fn read_input() -> Result<Vec<(Rule, String)>> {
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map(|line| {
                let captures = re.captures(&line).unwrap();

                let i = str::parse(&captures[1]).unwrap();
                let j = str::parse(&captures[2]).unwrap();
                let letter = *&captures[3].chars().nth(0).unwrap();
                let str = String::from(&captures[4]);

                (Rule { i, j, letter }, str)
            })
            .map_err(Into::into)
        })
        .collect()
}

fn part1(input: &[(Rule, String)]) -> usize {
    input
        .iter()
        .map(|(rule, password)| {
            let freq = password
                .chars()
                .filter(|&c| c == rule.letter)
                .map(|_| 1)
                .sum::<usize>();
            (rule.i..=rule.j).contains(&freq) as usize
        })
        .sum::<usize>()
}

fn part2(input: &[(Rule, String)]) -> usize {
    input
        .iter()
        .map(|(rule, password)| {
            let a = password.chars().nth(rule.i - 1).unwrap();
            let b = password.chars().nth(rule.j - 1).unwrap();
            ((a == rule.letter) ^ (b == rule.letter)) as usize
        })
        .sum::<usize>()
}
