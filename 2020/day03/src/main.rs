use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

struct Map(Vec<Vec<bool>>);

fn main() -> Result<()> {
    let input = read_input()?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn read_input() -> Result<Map> {
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map_err(Into::into)
                .map(|s| s.chars().map(|c| c == '#').collect::<Vec<_>>())
        })
        .collect::<Result<Vec<_>>>()
        .map(Map)
}

fn trees_for_slope(map: &Map, dx: usize, dy: usize) -> usize {
    (0..)
        .map(|i| (i * dy, (i * dx) % map.0[0].len()))
        .take_while(|(y, _)| *y < map.0.len())
        .map(|(y, x)| map.0[y][x] as usize)
        .sum()
}

fn part1(map: &Map) -> usize {
    trees_for_slope(map, 3, 1)
}

fn part2(map: &Map) -> usize {
    trees_for_slope(map, 1, 1)
        * trees_for_slope(map, 3, 1)
        * trees_for_slope(map, 5, 1)
        * trees_for_slope(map, 7, 1)
        * trees_for_slope(map, 1, 2)
}
