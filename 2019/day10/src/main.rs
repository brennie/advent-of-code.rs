use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::{Add, AddAssign, Index, IndexMut};
use std::f64;

use num::integer::gcd;

fn main() -> Result<(), Box<dyn Error>> {
    let asteroids = read_input()?;

    let visible = count_visible(&asteroids);

    let (p, count) = visible
        .iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();

    println!("part 1: {}", count);

    let q = vapourize(asteroids, *p);

    println!("part 2: {}", q.x * 100 + q.y);

    Ok(())
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Asteroids {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Asteroids {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        0 <= point.x && point.x < self.width as isize && 0 <= point.y && point.y < self.height as isize
    }
}

impl Index<Point> for Asteroids {
    type Output = bool;

    fn index(&self, idx: Point) -> &Self::Output {
        assert!(self.in_bounds(idx));

        &self.cells[idx.y as usize * self.width + idx.x as usize]
    }
}

impl IndexMut<Point> for Asteroids {
    fn index_mut(&mut self, idx: Point) -> &mut Self::Output {
        assert!(self.in_bounds(idx));

        &mut self.cells[idx.y as usize * self.width + idx.x as usize]
    }
}

fn read_input() -> Result<Asteroids, Box<dyn Error>> {
    let mut width = None;
    let mut cells = Vec::new();

    for line in BufReader::new(File::open("input")?).lines() {
        let line = line?;

        width.get_or_insert_with(|| line.len());

        assert_eq!(width.unwrap(), line.len());

        cells.extend(line.chars().map(|c| match c {
            '#' => true,
            '.' => false,
            _ => unimplemented!(),
        }));
    }

    let width = width.unwrap();

    Ok(Asteroids {
        width: width,
        height: cells.len() / width,
        cells,
    })
}

fn count_visible(asteroids: &Asteroids) -> HashMap<Point, usize> {
    let mut visible = HashMap::new();

    for y in 0..asteroids.height() as isize {
        for x in 0..asteroids.width() as isize {
            let p = Point { x, y };

            if !asteroids[p] {
                continue;
            }

            for v in 0..asteroids.height() as isize {
                for u in 0..asteroids.width() as isize {
                    let q = Point { x: u, y: v };

                    if p == q {
                        continue;
                    }

                    if asteroids[q] && is_visible(&asteroids, p, q) {
                        *visible.entry(p).or_default() += 1;
                    }
                }
            }
        }
    }

    visible
}

fn is_visible(asteroids: &Asteroids, from: Point, to: Point) -> bool {
    assert_ne!(from, to);

    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let g = gcd(dx, dy);

    let m = Point {
        x: dx / g,
        y: dy / g,
    };

    let mut p = from + m;

    loop {
        if p == to {
            break true;
        }

        if asteroids[p] {
            break false;
        }

        p += m;
    }
}

fn vapourize(mut asteroids: Asteroids, p: Point) -> Point {
    // Find all asteroids (other then the one at p) and compute their slopes and
    // their angle to the point p.
    let mut slopes = (0..asteroids.height())
        .flat_map(|y| (0..asteroids.width()).map(move |x| Point { x: x as isize, y: y as isize }))
        .filter_map(|q| {
            if p == q || !asteroids[q] {
                return None;
            }

            let dx = q.x - p.x;
            let dy = q.y - p.y;
            let g = gcd(dx, dy);

            let m = Point { x: dx / g, y: dy / g };

            // Shift the angle so that PI/2 is the minimum angle. This way,
            // sorting by angle will results in having the asteroids "up" first.
            let mut theta = f64::atan2(dy as f64, dx as f64);
            if theta < -f64::consts::FRAC_PI_2 {
                theta += f64::consts::PI + f64::consts::PI;
            }

            Some((m, theta))
        })
        .collect::<Vec<_>>();

    slopes.sort_by(|(_, alpha), (_, beta)| {
        // f64::atan2() has a range of (-PI, PI) and will never produce a NAN.
        PartialOrd::partial_cmp(&alpha, &beta).unwrap()
    });

    let mut last_slope = Point { x: 0, y: 0 };
    let mut count = 0;
    for (m, _) in slopes.iter().cycle() {
        if last_slope == *m {
            continue;
        }

        let mut q = p + *m;

        let fire = loop {
            if !asteroids.in_bounds(q) {
                break false;
            } else if asteroids[q] {
                break true;
            } else {
                q += *m;
            }
        };

        if fire {
            count += 1;
            asteroids[q] = false;

            if count == 200 {
                return q;
            }
        }

        last_slope = *m;
    }

    unreachable!();
}
