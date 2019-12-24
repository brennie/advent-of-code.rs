use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::{Index, IndexMut};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;

    {
        let mut seen = HashSet::new();
        seen.insert(input);
        let mut state = input;
        loop {
            state = simulate(&state);
            if seen.contains(&state) {
                println!("part 1: {}", state.biodiversity());
                break;
            } else {
                seen.insert(state);
            }
        }
    }

    {
        let mut states = HyperState::new(input);
        for _ in 0..200 {
            states = states.next();
        }

        println!("part 2: {}", states.bug_count());
    }

    Ok(())
}

fn read_input() -> Result<State, Box<dyn Error>> {
    let mut state = State::default();

    for (y, line) in BufReader::new(File::open("input")?).lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            state.0[y][x] = match c {
                '.' => false,
                '#' => true,
                _ => panic!(),
            };
        }
    }

    Ok(state)
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
struct State([[bool; 5]; 5]);

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

impl State {
    pub fn in_bounds(&self, idx: Point) -> bool {
        0 <= idx.x && (idx.x as usize) < 5 && 0 <= idx.y && (idx.y as usize) < 5
    }

    pub fn neighbours<'a>(&'a self, idx: Point) -> impl Iterator<Item = Point> + 'a {
        DIRECTIONS.iter().filter_map(move |d| {
            let p = idx + *d;

            if self.in_bounds(p) {
                Some(p)
            } else {
                None
            }
        })
    }
}

impl Index<Point> for State {
    type Output = bool;

    fn index(&self, idx: Point) -> &Self::Output {
        assert!(self.in_bounds(idx));

        &self.0[idx.y as usize][idx.x as usize]
    }
}

impl IndexMut<Point> for State {
    fn index_mut(&mut self, idx: Point) -> &mut Self::Output {
        assert!(self.in_bounds(idx));

        &mut self.0[idx.y as usize][idx.x as usize]
    }
}

fn simulate(state: &State) -> State {
    let mut new_state = State::default();

    for y in 0..5 {
        for x in 0..5 {
            let p = Point { x, y };
            let n_alive = state.neighbours(p).filter(|q| state[*q]).count();
            if state[p] && n_alive == 1 {
                new_state[p] = true;
            } else if !state[p] && 1 <= n_alive && n_alive <= 2 {
                new_state[p] = true;
            }
        }
    }

    new_state
}

impl State {
    fn biodiversity(&self) -> u64 {
        let mut sum = 0;
        for y in 0..5 {
            for x in 0..5 {
                let idx = y as u32 * 5 + x as u32;

                let p = Point { x, y };

                if self[p] {
                    sum += 2u64.pow(idx);
                }
            }
        }
        sum
    }

    fn bug_count(&self) -> u64 {
        let mut sum = 0;
        for y in 0..5 {
            for x in 0..5 {
                if self[Point { x, y }] {
                    sum += 1;
                }
            }
        }
        sum
    }
}

#[derive(Default)]
struct HyperState(HashMap<isize, State>);

impl HyperState {
    fn new(st: State) -> Self {
        let mut sts = HashMap::new();
        sts.insert(0, st);

        HyperState(sts)
    }

    pub fn bug_count(&self) -> u64 {
        self.0.values().map(|st| st.bug_count()).sum::<u64>()
    }

    pub fn next(self) -> HyperState {
        let mut next_state = HyperState::default();
        let mut levels = self.0.keys().cloned().collect::<Vec<_>>();
        levels.push(levels.iter().min().unwrap() - 1);
        levels.push(levels.iter().max().unwrap() + 1);

        for l in levels {
            next_state.0.insert(l, State::default());

            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }

                    let p = Point { x, y };

                    let mut alive = 0;
                    for (nl, np) in next_state.neighbours(l, p) {
                        if self.0.contains_key(&nl) && self.0[&nl][np] {
                            alive += 1;
                        }
                    }

                    let c = self.0.contains_key(&l) && self.0[&l][p];
                    if c && alive == 1 {
                        next_state.0.get_mut(&l).unwrap()[p] = true;
                    } else if !c && 1 <= alive && alive <= 2 {
                        next_state.0.get_mut(&l).unwrap()[p] = true;
                    }
                }
            }
        }

        next_state
    }

    fn neighbours(&self, level: isize, p: Point) -> Vec<(isize, Point)> {
        let mut ns: Vec<(isize, Point)> = self.0[&level]
            .neighbours(p)
            .filter(|q| *q != Point { x: 2, y: 2 })
            .map(|q| (level, q))
            .collect();

        // An tile adjacent to the outer space.
        if p.y == 0 || p.y == 4 || p.x == 0 || p.x == 4 {
            if p.y == 0 {
                ns.push((level - 1, Point { x: 2, y: 1 }));
            } else if p.y == 4 {
                ns.push((level - 1, Point { x: 2, y: 3 }));
            }

            if p.x == 0 {
                ns.push((level - 1, Point { x: 1, y: 2 }));
            } else if p.x == 4 {
                ns.push((level - 1, Point { x: 3, y: 2 }));
            }
        }

        // A tile adjacent to the inner space.
        if ((p.y == 1 || p.y == 3) && p.x == 2) || ((p.x == 1 || p.x == 3) && p.y == 2) {
            match p {
                Point { x: 2, y: 1 } => {
                    for x_ in 0..5 {
                        ns.push((level + 1, Point { x: x_, y: 0 }));
                    }
                }
                Point { x: 2, y: 3 } => {
                    for x_ in 0..5 {
                        ns.push((level + 1, Point { x: x_, y: 4 }));
                    }
                }
                Point { x: 1, y: 2 } => {
                    for y_ in 0..5 {
                        ns.push((level + 1, Point { x: 0, y: y_ }));
                    }
                }
                Point { x: 3, y: 2 } => {
                    for y_ in 0..5 {
                        ns.push((level + 1, Point { x: 4, y: y_ }));
                    }
                }
                _ => panic!(),
            }
        }
        ns
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_hyperstate_neighbours() {
        let h = HyperState::new(State::default());
        assert_eq!(
            h
                .neighbours(0, Point { x: 1, y: 1 })
                .iter()
                .collect::<HashSet<_>>(),
            [
                (0, Point { x: 0, y: 1 }),
                (0, Point { x: 2, y: 1 }),
                (0, Point { x: 1, y: 0 }),
                (0, Point { x: 1, y: 2 }),
            ]
            .iter()
            .collect::<HashSet<_>>()
        );

        assert_eq!(
            h.neighbours(0, Point { x: 0, y: 0 })
            .iter()
            .collect::<HashSet<_>>(),
            [
                (0, Point {x: 0, y: 1}),
                (0, Point {x: 1, y: 0}),
                (-1, Point {x : 2, y: 1 }),
                (-1, Point {x: 1, y: 2}),
            ]
            .iter().collect::<HashSet<_>>()
        );

        assert_eq!(
            h.neighbours(0, Point { x: 2, y: 1})
            .iter()
            .collect::<HashSet<_>>(),
            [
                (0, Point {x: 1, y: 1}),
                (0, Point {x: 3, y: 1}),
                (0, Point {x: 2, y: 0}),
                (1, Point {x: 0, y: 0}),
                (1, Point {x: 1, y: 0}),
                (1, Point {x: 2, y: 0}),
                (1, Point {x: 3, y: 0}),
                (1, Point {x: 4, y: 0}),
            ]
            .iter()
            .collect::<HashSet<_>>()
        );
    }
}
