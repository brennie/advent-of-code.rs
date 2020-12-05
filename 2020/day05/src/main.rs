
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

fn read_input() -> Result<Vec<String>> {
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| r.map_err(Into::into))
        .map(|r: Result<_>| r)
        .collect()
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| Ticket::from_bsp(s).id())
        .max()
        .unwrap()
}

fn part2(input: &[String]) -> u32 {
    let mut seats = HashSet::new();

    for row in 0..128 {
        for column in 0..8 {
            seats.insert(Ticket { row, column });
        }
    }

    let tickets = input
        .iter()
        .map(|s| Ticket::from_bsp(s))
        .collect::<HashSet<Ticket>>();

    for ticket in &tickets {
        seats.remove(&ticket);
    }

    for seat in seats.iter() {
        // We know we are not in the front or back row and that we have two neighbours.
        if seat.row == 0 || seat.row == 127 || seat.column == 0 || seat.column == 1 {
            continue;
        }

        let left = Ticket {
            row: seat.row,
            column: seat.column - 1,
        };
        let right = Ticket {
            row: seat.row,
            column: seat.column + 1,
        };

        if tickets.contains(&left) && tickets.contains(&right) {
            return seat.id();
        }
    }

    unreachable!()
}

#[derive(Hash, Eq, Debug, PartialEq)]
struct Ticket {
    row: u32,
    column: u32,
}

impl Ticket {
    fn from_bsp(s: &str) -> Ticket {
        assert!(s.len() == 10);

        let row = binary_search(0, 128, s.chars().take(7).map(Clue::from_fb));
        let column = binary_search(0, 8, s.chars().skip(7).take(3).map(Clue::from_lr));

        Ticket { row, column }
    }

    fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

fn binary_search(min: u32, max: u32, clues: impl Iterator<Item = Clue>) -> u32 {
    let mut min = min;
    let mut max = max;
    for clue in clues {
        let midpoint = (min + max) / 2;
        match clue {
            Clue::Lower => max = midpoint,
            Clue::Higher => min = midpoint,
        }
    }

    min
}

enum Clue {
    Lower,
    Higher,
}

impl Clue {
    fn from_lr(c: char) -> Clue {
        match c {
            'L' => Clue::Lower,
            'R' => Clue::Higher,
            c => panic!("expected L or R, got {}", c)
        }
    }

    fn from_fb(c: char) -> Clue {
        match c {
            'F' => Clue::Lower,
            'B' => Clue::Higher,
            c => panic!("expected F or B, got {}", c),
        }
    }
}

