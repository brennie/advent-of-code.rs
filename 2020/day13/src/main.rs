use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let input = read_input();

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input.1));

    Ok(())
}

fn read_input() -> (isize, Vec<Option<isize>>) {
    let mut lines = BufReader::new(File::open("input").unwrap()).lines();

    let ts = lines.next().unwrap().unwrap().parse().unwrap();

    let ids = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect();

    (ts, ids)
}

fn part1(input: &(isize, Vec<Option<isize>>)) -> isize {
    let ts = input.0;
    let ids = &(input.1);

    let ids = ids.iter().filter_map(|x| *x).collect::<Vec<_>>();

    let (id, next) = ids
        .iter()
        .map(|id| {
            let last = (ts / id);
            let mut next = last * id;
            while next < ts {
                next += id;
            }
            (id, next)
        })
        .min_by_key(|(_, next)| *next)
        .unwrap();

    (next - ts) * id
}

struct Timing {
    id: i128,
    offset: i128,
}

fn part2(input: &[Option<isize>]) -> i128 {
    solve_crt(
        &input
            .iter()
            .enumerate()
            .filter_map(|(offset, id)| {
                id.map(|id| Timing {
                    id: id as i128,
                    offset: offset as i128,
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn modular_exp(b: i128, p: i128, m: i128) -> i128 {
    let mut b = if b > 0 { b } else { b + m };
    let mut p = p;

    let mut result = 1;

    while p > 0 {
        if p % 2 == 1 {
            result = (result * b) % m;
            p -= 1;
        }

        p /= 2;
        b = (b * b) % m;
    }

    result
}

fn inverse_mod_p(x: i128, p: i128) -> i128 {
    modular_exp(x, p - 2, p)
}

fn solve_crt(timings: &[Timing]) -> i128 {
    let big_m = timings.iter().map(|t| t.id).fold(1, |acc, x| acc * x);

    let mut x = 0;

    for t in timings {
        let ai = (t.id - t.offset) % t.id;

        let mi = big_m / t.id;
        let mi_inv_mod_id = inverse_mod_p(mi, t.id);

        x += (ai * mi * mi_inv_mod_id) % big_m;

        while x < 0 {
            x += big_m;
        }
    }
    x
}
