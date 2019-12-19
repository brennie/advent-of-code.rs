use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;
use std::ops::Mul;

use derive_more::{Add, AddAssign};
use itertools::Itertools;

mod intcode;

use crate::intcode::Vm;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Add, AddAssign)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Point {
    pub fn rotate_right(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn rotate_left(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.y >= 0 && self.x >= 0 && (self.x as usize) < width && (self.y as usize) < height
    }
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

const DIRECTIONS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    let view = {
        let mut vm = Vm::new(&mem);

        let mut view = Vec::new();
        'outer: loop {
            let mut row = Vec::new();
            loop {
                let output = match vm.run() {
                    Some(output) => output,
                    None => break 'outer,
                };

                match output as u8 as char {
                    '\n' => {
                        if row.len() > 0 {
                            view.push(row);
                            break;
                        } else {
                            break 'outer;
                        }
                    }
                    c => row.push(c),
                }
            }
        }
        for row in &view {
            for col in row {
                print!("{}", col);
            }
            println!()
        }

        let mut sum = 0;
        for y in 1..view.len() - 1 {
            for x in 1..view[0].len() - 1 {
                if view[y][x] == '#'
                    && view[y][x - 1] == '#'
                    && view[y][x + 1] == '#'
                    && view[y - 1][x] == '#'
                    && view[y + 1][x] == '#'
                {
                    sum += x * y;
                }
            }
        }

        println!("part 1: {}", sum);

        view
    };

    {
        let width = view[0].len();
        let height = view.len();

        let mut pos = None;
        let mut direction = None;

        for y in 0..height {
            for x in 0..width {
                let c = view[y][x];
                if "^v<>".contains(c) {
                    pos = Some(Point {
                        x: x as i64,
                        y: y as i64,
                    });
                    direction = Some(match c {
                        '^' => UP,
                        'v' => DOWN,
                        '<' => LEFT,
                        '>' => RIGHT,
                        _ => unreachable!(),
                    });
                    break;
                }
            }
        }

        let mut pos = pos.unwrap();
        let mut direction = direction.unwrap();
        let mut path = Vec::new();

        loop {
            let mut found = false;
            for d in &DIRECTIONS {
                if d.rotate_left().rotate_left() == direction {
                    continue;
                }

                let p: Point = pos + *d;
                if p.in_bounds(width, height) && view[p.y as usize][p.x as usize] == '#' {
                    found = true;
                    if direction.rotate_left() == *d {
                        path.push("L".into());
                    } else if direction.rotate_right() == *d {
                        path.push("R".into());
                    } else {
                        panic!("L/R expected");
                    }

                    direction = *d;
                    break;
                }
            }
            if !found {
                break;
            }

            let mut count = 0;
            loop {
                let p: Point = pos + direction;
                if p.in_bounds(width, height) && view[p.y as usize][p.x as usize] == '#' {
                    pos = p;
                    count += 1;
                } else {
                    break;
                }
            }
            path.push(count.to_string());
        }

        {
            let path: String = path.into_iter().intersperse(",".into()).collect();
            let (main, a, b, c) = compress_path(&path).unwrap();

            let input = [&main, a, b, c, "n"]
                .into_iter()
                .cloned()
                .intersperse("\n")
                .chain(once("\n"))
                .flat_map(|s| s.bytes().map(|c| c as isize))
                .collect::<Vec<_>>();

            let mut mem = mem.clone();
            mem[0] = 2;

            let mut vm = intcode::Vm::new_with_input(&mem, &input);

            let mut output = None;
            loop {
                match vm.run() {
                    Some(o) => output = Some(o),
                    None => break,
                };
            }

            println!("{}", output.unwrap());
        }

        // println!("path = {:?}", path);
    }

    Ok(())
}

fn read_input() -> Result<Vec<isize>, Box<dyn Error>> {
    let mut buf = String::new();
    File::open("input")?.read_to_string(&mut buf)?;
    buf[..buf.len() - 1]
        .split(',')
        .map(|s| str::parse::<isize>(&s).map_err(Into::into))
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Chunk<'a> {
    Compressed(char),
    Uncompressed(&'a str),
}

fn find_prefix(path: &str, max_len: usize) -> Option<&str> {
    let mut end = None;

    loop {
        let mut start = end.unwrap_or(0);
        start = match path[start..].find(',') {
            Some(idx) => start + idx + 1,
            None => break,
        };
        let second_comma = match path[start..].find(',') {
            Some(idx) => start + idx,
            None => break,
        };

        if second_comma > max_len {
            break;
        }

        end = Some(second_comma + 1);
    }

    end.map(|end| &path[..end - 1])
}

fn rechunk<'a>(old: &[Chunk<'a>], comp: &'a str, name: char) -> Vec<Chunk<'a>> {
    let mut new = vec![];

    for chunk in old {
        match chunk {
            Chunk::Compressed(..) => new.push(*chunk),

            Chunk::Uncompressed(s) => {
                let mut s = &s[..];
                while let Some(idx) = s.find(comp) {
                    let end = idx + comp.len();
                    if idx > 0 {
                        new.push(Chunk::Uncompressed(&s[..idx]));
                    }
                    new.push(Chunk::Compressed(name));

                    s = &s[end..];
                    if s.starts_with(',') {
                        s = &s[1..];
                    }
                }

                if s.starts_with(',') {
                    s = &s[1..];
                }

                if s.len() > 0 {
                    new.push(Chunk::Uncompressed(s));
                }
            }
        }
    }

    new
}

fn compress_path(path: &str) -> Option<(String, &str, &str, &str)> {
    'a: for max_a in (0..=20).rev() {
        let a = find_prefix(path, max_a)?;

        let chunks = rechunk(&[Chunk::Uncompressed(&path)], a, 'A');

        let s = chunks
            .iter()
            .filter_map(|c| match c {
                Chunk::Uncompressed(s) => Some(s),
                _ => None,
            })
            .next()
            .unwrap();

        'b: for max_b in (0..=20).rev() {
            let b = match find_prefix(s, max_b) {
                Some(b) => b,
                None => continue 'a,
            };

            let chunks = rechunk(&chunks, b, 'B');

            let s = chunks
                .iter()
                .filter_map(|c| match c {
                    Chunk::Uncompressed(s) => Some(s),
                    _ => None,
                })
                .next()
                .unwrap();

            'c: for max_c in (0..=20).rev() {
                let c = match find_prefix(s, max_c) {
                    Some(c) => c,
                    None => continue 'b,
                };

                let chunks = rechunk(&chunks, c, 'C');

                // If there are any uncompressed chunks, we failed to compress
                // with three components.
                if chunks.iter().any(|c| match c {
                    Chunk::Uncompressed(..) => true,
                    _ => false,
                }) {
                    continue 'b;
                }

                let main = chunks
                    .into_iter()
                    .map(|c| match c {
                        Chunk::Compressed(name) => name,
                        _ => unreachable!(),
                    })
                    .intersperse(',')
                    .collect::<String>();

                if main.len() > 20 {
                    // Our compression was not good enough so the main program
                    // is too long.
                    continue 'b;
                }

                return Some((main, a, b, c));
            }
        }
    }

    None
}
