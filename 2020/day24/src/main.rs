use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use either::Either;

#[derive(Clone, Copy)]
enum Direction {
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

const DIRECTIONS: [Direction; 6] = [
    Direction::West,
    Direction::East,
    Direction::NorthWest,
    Direction::NorthEast,
    Direction::SouthWest,
    Direction::SouthEast,
];

fn main() -> Result<()> {
    let direction_sets = read_input()?;

    let mut tiles = flip_tiles(&direction_sets);
    println!(
        "part 1: {}",
        tiles
            .values()
            .cloned()
            .filter(|&t| t == Tile::Black)
            .count()
    );

    for _ in 0..100 {
        tiles = evolve_tiles(&tiles);
    }

    println!(
        "part 2: {}",
        tiles
            .values()
            .cloned()
            .filter(|&t| t == Tile::Black)
            .count()
    );

    Ok(())
}

fn read_input() -> Result<Vec<Vec<Direction>>> {
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map_err(Into::into).map(|line| {
                let mut ds = Vec::new();
                let mut prev = None;
                for c in line.chars() {
                    match (prev, c) {
                        (None, 'w') => ds.push(Direction::West),
                        (None, 'e') => ds.push(Direction::East),
                        (None, 's') => prev = Some('s'),
                        (None, 'n') => prev = Some('n'),
                        (Some('n'), 'w') => {
                            prev = None;
                            ds.push(Direction::NorthWest);
                        }
                        (Some('n'), 'e') => {
                            prev = None;
                            ds.push(Direction::NorthEast);
                        }
                        (Some('s'), 'w') => {
                            prev = None;
                            ds.push(Direction::SouthWest);
                        }
                        (Some('s'), 'e') => {
                            prev = None;
                            ds.push(Direction::SouthEast);
                        }
                        _ => panic!(),
                    }
                }

                ds
            })
        })
        .collect()
}

#[derive(Clone, Copy, Default, Debug, Eq, Hash, PartialEq)]
pub struct HexCoord {
    x: isize,
    y: isize,
    z: isize,
}

impl HexCoord {
    fn move_direction(&self, direction: Direction) -> Self {
        let mut coord = *self;
        match direction {
            Direction::East => {
                coord.x += 1;
                coord.y -= 1;
            }
            Direction::West => {
                coord.x -= 1;
                coord.y += 1;
            }
            Direction::NorthEast => {
                coord.x += 1;
                coord.z -= 1;
            }
            Direction::NorthWest => {
                coord.y += 1;
                coord.z -= 1;
            }
            Direction::SouthEast => {
                coord.y -= 1;
                coord.z += 1;
            }
            Direction::SouthWest => {
                coord.x -= 1;
                coord.z += 1;
            }
        }
        coord
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    White,
    Black,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::White
    }
}

impl Tile {
    fn flip(&mut self) {
        match self {
            Tile::White => *self = Tile::Black,
            Tile::Black => *self = Tile::White,
        }
    }
}

fn flip_tiles(direction_sets: &[Vec<Direction>]) -> HashMap<HexCoord, Tile> {
    let mut tiles = HashMap::<HexCoord, Tile>::new();

    for directions in direction_sets {
        let mut position = HexCoord::default();

        for direction in directions {
            position = position.move_direction(*direction);
        }

        tiles.entry(position).or_default().flip()
    }

    tiles
}

fn evolve_tiles(tiles: &HashMap<HexCoord, Tile>) -> HashMap<HexCoord, Tile> {
    let mut next = HashMap::new();

    let neighbouring_white_tiles: HashSet<HexCoord> = tiles
        .iter()
        .flat_map(|(&coord, &tile)| {
            if tile == Tile::Black {
                Either::Left(
                    DIRECTIONS
                        .iter()
                        .map(move |&direction| coord.move_direction(direction))
                        .filter(|&coord| {
                            tiles.get(&coord).cloned().unwrap_or_default() == Tile::White
                        }),
                )
            } else {
                Either::Right(std::iter::empty())
            }
        })
        .collect();

    // Black tiles
    for coord in tiles
        .iter()
        .filter(|(_, &tile)| tile == Tile::Black)
        .map(|(&coord, _)| coord)
    {
        let neighbours = DIRECTIONS
            .iter()
            .map(|&direction| coord.move_direction(direction))
            .filter(|&coord| tiles.get(&coord).cloned().unwrap_or_default() == Tile::Black)
            .count();

        if 0 < neighbours && neighbours <= 2 {
            next.insert(coord, Tile::Black);
        }
    }

    for &coord in neighbouring_white_tiles.iter() {
        let neighbours = DIRECTIONS
            .iter()
            .map(|&direction| coord.move_direction(direction))
            .filter(|&coord| tiles.get(&coord).cloned().unwrap_or_default() == Tile::Black)
            .count();

        if neighbours == 2 {
            next.insert(coord, Tile::Black);
        }
    }

    next
}
