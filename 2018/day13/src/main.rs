use std::cmp::Ordering;
use std::error::Error;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem::replace;
use std::ops::{Index, IndexMut};

use derive_more::{Add, AddAssign};

const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;

    {
        let mut system = input.clone();
        loop {
            if let Some(position) = system.next().iter().next() {
                println!("part 1: {},{}", position.x, position.y);
                break;
            }
        }
    }

    {
        let mut system = input;
        loop {
            system.next();
            if system.carts().count() == 1 {
                let position = system.carts().next().unwrap().position;
                println!("part 2: {},{}", position.x, position.y);
                break;
            }
        }
    }

    Ok(())
}

#[derive(Add, AddAssign, Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Ord for Point {
    fn cmp(&self, rhs: &Point) -> Ordering {
        self.y.cmp(&rhs.y).then(self.x.cmp(&rhs.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Point) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}


impl Point {
    pub fn rotate_right(self) -> Self {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rotate_left(self) -> Self {
        Point {
            x: self.y,
            y: -self.x,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Decision {
    Left,
    Straight,
    Right,
}

impl Decision {
    fn next(&mut self) -> Self {
        let decision = *self;
        match self {
            Decision::Left => {
                *self = Decision::Straight;
            }
            Decision::Straight => {
                *self = Decision::Right;
            }
            Decision::Right => {
                *self = Decision::Left;
            }
        }

        decision
    }
}

impl Default for Decision {
    fn default() -> Self {
        Decision::Left
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cart {
    pub position: Point,
    velocity: Point,
    next_intersection: Decision,
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, rhs: &Cart) -> Option<Ordering> {
        self.position.partial_cmp(&rhs.position)
    }
}

impl Ord for Cart {
    fn cmp(&self, rhs: &Cart) -> Ordering {
        self.position.cmp(&rhs.position)
    }
}

impl Cart {
    pub fn new(position: Point, velocity: Point) -> Cart {
        Cart {
            position,
            velocity,
            next_intersection: Decision::default(),
        }
    }

    pub fn next(&mut self, g: &Grid) {
        self.velocity = match g[self.position] {
            b'+' => match self.next_intersection.next() {
                Decision::Left => self.velocity.rotate_left(),
                Decision::Right => self.velocity.rotate_right(),
                Decision::Straight => self.velocity,
            },

            b'|' | b'-' => self.velocity,

            b'/' => match self.velocity {
                UP | DOWN => self.velocity.rotate_right(),
                LEFT | RIGHT => self.velocity.rotate_left(),
                _ => unreachable!(),
            },
            b'\\' => match self.velocity {
                UP | DOWN => self.velocity.rotate_left(),
                LEFT | RIGHT => self.velocity.rotate_right(),
                _ => unreachable!(),
            },
            c => panic!(format!("unexpected {}", c)),
        };

        self.position += self.velocity;
    }
}

#[derive(Clone)]
struct Grid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn in_bounds(&self, idx: Point) -> bool {
        0 <= idx.x && (idx.x as usize) < self.width && 0 <= idx.y && (idx.y as usize) < self.height
    }
}

impl Index<Point> for Grid {
    type Output = u8;
    fn index(&self, idx: Point) -> &Self::Output {
        assert!(self.in_bounds(idx));
        let y = idx.y as usize;
        let x = idx.x as usize;

        &self.cells[y * self.width + x]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, idx: Point) -> &mut Self::Output {
        assert!(self.in_bounds(idx));

        let y = idx.y as usize;
        let x = idx.x as usize;

        &mut self.cells[y * self.width + x]
    }
}

#[derive(Clone)]
struct System {
    grid: Grid,
    carts: BTreeSet<Cart>,
}

impl System {
    pub fn next(&mut self) -> Vec<Point> {
        let mut crashes = vec![];
        let mut prev_carts = replace(&mut self.carts, BTreeSet::new());

        while prev_carts.len() > 0 {
            let mut cart = prev_carts.iter().cloned().next().unwrap();
            prev_carts.remove(&cart);

            cart.next(&self.grid);

            if let Some(cart2) = prev_carts.iter().cloned().find(|cart2| cart.position == cart2.position) {
                prev_carts.remove(&cart2);
                crashes.push(cart.position);
            } else if let Some(cart2) = self.carts.iter().cloned().find(|cart2| cart.position == cart2.position) {
                self.carts.remove(&cart2);
                crashes.push(cart.position);
            } else {
                self.carts.insert(cart);
            }
        }

        crashes
    }

    pub fn carts(&self) -> impl Iterator<Item = &'_ Cart> + '_ {
        self.carts.iter()
    }
}

fn read_input() -> Result<System, Box<dyn Error>> {
    let mut grid = {
        let mut height = 0;
        let mut cells = vec![];

        for line in BufReader::new(File::open("input")?).lines() {
            let line = line?;
            cells.extend(line.as_bytes());
            height += 1;
        }

        assert_ne!(height, 0);

        let width = cells.len() / height;
        Grid {
            cells,
            width,
            height,
        }
    };

    let mut carts = BTreeSet::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point {
                x: x as isize,
                y: y as isize,
            };
            let c = grid[p];

            match c {
                b'-' | b'<' | b'>' => {
                    if c == b'<' || c == b'>' {
                        carts.insert(Cart::new(p, if c == b'<' { LEFT } else { RIGHT }));
                        grid[p] = b'-';
                    }
                }
                b'|' | b'v' | b'^' => {
                    if c == b'v' || c == b'^' {
                        carts.insert(Cart::new(p, if c == b'v' { DOWN } else { UP }));
                        grid[p] = b'|';
                    }
                }

                b'+' | b'/' | b'\\' | b' ' => {}

                _ => panic!(),
            }
        }
    }

    Ok(System { grid, carts })
}
