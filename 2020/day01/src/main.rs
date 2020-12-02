use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let expenses = read_input()?;

    println!("part 1: {}", part1(&expenses).unwrap());
    println!("part 2: {}", part2(&expenses).unwrap());

    Ok(())
}

fn read_input() -> Result<Vec<i32>> {
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| r
            .map_err(Into::into)
            .and_then(|s| str::parse::<i32>(&s).map_err(Into::into)))
        .collect()
}

fn part1(expenses: &[i32]) -> Option<i32> {
    for (i, expense_a) in expenses.iter().enumerate() {
        for (j, expense_b) in expenses.iter().enumerate() {
            if i == j {
                continue;
            }

            if expense_a + expense_b == 2020 {
                return Some(expense_a * expense_b);
            }
        }
    }

    None
}

fn part2(expenses: &[i32]) -> Option<i32> {
    for (i, expense_a) in expenses.iter().enumerate() {
        for (j, expense_b) in expenses.iter().enumerate() {
            for (k, expense_c) in expenses.iter().enumerate() {
                if i == j || j == k || i == k {
                    continue;
                }

                if expense_a + expense_b + expense_c== 2020 {
                    return Some(expense_a * expense_b * expense_c);
                }
            }
        }
    }

    None
}
