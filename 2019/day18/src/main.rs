use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::{Index, IndexMut};

use derive_more::{Add, AddAssign};

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const DIRECTIONS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Add, AddAssign, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    keys: HashMap<usize, Point>,
    entrance: Vec<Point>,
}

impl Map {
    pub fn in_bounds(&self, idx: Point) -> bool {
        0 <= idx.x && (idx.x as usize) < self.width && 0 <= idx.y && (idx.y as usize) < self.height
    }

    pub fn neighbours<'a>(&'a self, p: Point, keys: usize) -> impl Iterator<Item = Point> + 'a {
        DIRECTIONS.iter().filter_map(move |d: &Point| {
            let q = p + *d;

            if self.in_bounds(q) && self[q].accessible(keys) {
                Some(q)
            } else {
                None
            }
        })
    }
}

impl Index<Point> for Map {
    type Output = Tile;

    fn index(&self, idx: Point) -> &Self::Output {
        assert!(self.in_bounds(idx));

        &self.tiles[(idx.y as usize) * self.width + idx.x as usize]
    }
}

impl IndexMut<Point> for Map {
    fn index_mut(&mut self, idx: Point) -> &mut Self::Output {
        assert!(self.in_bounds(idx));

        &mut self.tiles[(idx.y as usize) * self.width + idx.x as usize]
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Tile {
    Wall,
    Floor,
    Entrance,
    Key(usize),
    Door(usize),
}

impl Tile {
    fn accessible(&self, keys: usize) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Door(mask) => mask & keys != 0,
            _ => true,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Floor,
            '@' => Tile::Entrance,
            'a'..='z' => {
                let k = (c as u8 - b'a') as usize;
                Tile::Key(2 << k)
            }
            'A'..='Z' => {
                let k = (c as u8 - b'A') as usize;
                Tile::Door(2 << k)
            }
            _ => unimplemented!("invalid tile"),
        }
    }
}

#[derive(Clone, Debug, Eq)]
struct State {
    pos: Vec<Point>,
    keys: usize,
    key_count: usize,
    cost: u64,
}

impl PartialEq for State {
    fn eq(&self, rhs: &State) -> bool {
        self.pos == rhs.pos && self.key_count == rhs.key_count && self.cost == rhs.cost
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, rhs: &State) -> Option<Ordering> {
        // Order by cost decreasing and number of keys increasing.
        Some(
            self.cost
                .cmp(&rhs.cost)
                .reverse()
                .then(self.key_count.cmp(&rhs.key_count)),
        )
    }
}

impl Ord for State {
    fn cmp(&self, rhs: &State) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = read_input()?;

    println!("part 1: {}", find_keys(&map));

    {
        let entrance = map.entrance[0];
        map.entrance = vec![
            entrance + UP + LEFT,
            entrance + UP + RIGHT,
            entrance + DOWN + LEFT,
            entrance + DOWN + RIGHT,
        ];

        // We don't actually need to mark the map entrances as Tile::Entrance since
        // we keep track of them separately.
        map[entrance] = Tile::Wall;
        for direction in &DIRECTIONS {
            map[entrance + *direction] = Tile::Wall;
        }

        println!("part 2: {}", find_keys(&map));
    }

    Ok(())
}

fn read_input() -> Result<Map, Box<dyn Error>> {
    let mut tiles = Vec::new();
    let mut width = None;
    let mut height = 0usize;
    let mut entrance = None;
    let mut keys = HashMap::new();

    for line in BufReader::new(File::open("input")?).lines() {
        let line = line?;

        for (x, c) in line.chars().enumerate() {
            let tile: Tile = c.into();
            let p = Point {
                x: x as isize,
                y: height as isize,
            };

            match tile {
                Tile::Key(k) => {
                    keys.insert(k, p);
                }
                Tile::Entrance => entrance = Some(p),
                _ => (),
            }

            tiles.push(tile);
        }

        width = Some(line.len());
        height += 1;
    }

    Ok(Map {
        width: width.unwrap(),
        height,
        tiles,
        keys,
        entrance: vec![entrance.unwrap()],
    })
}

fn find_keys(map: &Map) -> u64 {
    let num_robots = map.entrance.len();

    let mut states = vec![BinaryHeap::new(); num_robots];
    let mut closed = HashSet::<(Vec<Point>, usize)>::new();

    for robot in 0..num_robots {
        states[robot].push(State {
            pos: map.entrance.clone(),
            keys: 0,
            key_count: 0,
            cost: 0,
        });
    }
    closed.insert((map.entrance.clone(), 0));

    loop {
        for robot in 0..num_robots {
            // Best-first search.
            while let Some(state) = states[robot].pop() {
                let cost = state.cost + 1;

                for next_robot_pos in map.neighbours(state.pos[robot], state.keys) {
                    let mut keys = state.keys;
                    let mut key_count = state.key_count;

                    if let Tile::Key(k) = map[next_robot_pos] {
                        if keys & k == 0 {
                            keys |= k;
                            key_count += 1;

                            if key_count == map.keys.len() {
                                return cost;
                            }
                        }
                    }

                    let mut next_pos = state.pos.clone();
                    next_pos[robot] = next_robot_pos;

                    if !closed.contains(&(next_pos.clone(), keys)) {
                        closed.insert((next_pos.clone(), keys));

                        let next_state = State {
                            pos: next_pos,
                            keys,
                            key_count,
                            cost,
                        };

                        if let Tile::Key(..) = map[next_robot_pos] {
                            // If we find a key, then we can unblock any other robots.
                            for robot in 0..num_robots {
                                states[robot].push(next_state.clone());
                            }
                        } else {
                            states[robot].push(next_state);
                        }
                    }
                }
            }
        }
    }
}
