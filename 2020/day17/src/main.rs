use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use itertools::Itertools;

#[derive(Clone)]
struct Grid {
    /// The x, y coordinate of top-let cell
    offset: (isize, isize),

    /// The width and height.
    size: isize,

    /// The cells.
    cells: Vec<bool>,
}

impl Grid {
    fn new(size: isize) -> Grid {
        Grid {
            offset: (0, 0),
            size: size,
            cells: vec![false; (size * size) as usize],
        }
    }

    fn with_offset(size: isize, offset: (isize, isize)) -> Grid {
        Grid {
            offset: offset,
            size: size,
            cells: vec![false; (size * size) as usize],
        }
    }

    fn left(&self) -> isize {
        self.offset.0
    }

    fn top(&self) -> isize {
        self.offset.1
    }

    fn right(&self) -> isize {
        self.offset.0 + self.size - 1
    }

    fn bottom(&self) -> isize {
        self.offset.1 + self.size - 1
    }

    fn alive(&self, (x, y): (isize, isize)) -> bool {
        if x < self.left() || x > self.right() || y < self.top() || y > self.bottom() {
            return false;
        }

        let u = x - self.left();
        let v = y - self.top();

        assert!(u >= 0 && u < self.size);
        assert!(v >= 0 && v < self.size);

        self.cells[(v * self.size + u) as usize]
    }
}

impl std::ops::Index<(isize, isize)> for Grid {
    type Output = bool;
    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        assert!(x >= self.left() && x <= self.right());
        assert!(y >= self.top() && y <= self.bottom());

        let u = x - self.left();
        let v = y - self.top();

        assert!(u >= 0 && u < self.size);
        assert!(v >= 0 && v < self.size);

        &self.cells[(v * self.size + u) as usize]
    }
}

impl std::ops::IndexMut<(isize, isize)> for Grid {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        assert!(x >= self.left() && x <= self.right());
        assert!(y >= self.top() && y <= self.bottom());

        let u = x - self.left();
        let v = y - self.top();

        assert!(u >= 0 && u < self.size);
        assert!(v >= 0 && v < self.size);

        &mut self.cells[(v * self.size + u) as usize]
    }
}

#[derive(Clone)]
struct PocketDimension {
    // Negative grids, with grid_n[i] corresponding to z = -1 - i.
    grid_n: Vec<Grid>,

    /// The z=0 grid.
    grid_0: Grid,

    /// Positive z grids, with grid_p[i] corresponding to z = 1 + i.
    grid_p: Vec<Grid>,
}

impl PocketDimension {
    fn cells(&self) -> impl Iterator<Item = (isize, isize, isize)> {
        let min_x = self.grid_0.left();
        let max_x = self.grid_0.right();

        let min_y = self.grid_0.top();
        let max_y = self.grid_0.bottom();

        let min_z = -1 * self.grid_n.len() as isize;
        let max_z = self.grid_p.len() as isize;

        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .cartesian_product(min_z..=max_z)
            .map(|((x, y), z)| (x, y, z))
    }

    fn alive(&self, (x, y, z): (isize, isize, isize)) -> bool {
        if z == 0 {
            self.grid_0.alive((x, y))
        } else if z > 0 {
            let i = z - 1;
            if i >= self.grid_p.len() as isize {
                false
            } else {
                self.grid_p[i as usize].alive((x, y))
            }
        } else {
            let i = -z - 1;
            if i >= self.grid_n.len() as isize {
                false
            } else {
                self.grid_n[i as usize].alive((x, y))
            }
        }
    }

    fn grow(&self) -> PocketDimension {
        let grid_0 = Grid::with_offset(
            self.grid_0.size + 2,
            (self.grid_0.offset.0 - 1, self.grid_0.offset.1 - 1),
        );

        let mut grid_n = Vec::with_capacity(self.grid_n.len() + 1);
        let mut grid_p = Vec::with_capacity(self.grid_p.len() + 1);

        while grid_n.len() < self.grid_n.len() + 1 {
            grid_n.push(grid_0.clone());
        }

        while grid_p.len() < self.grid_p.len() + 1 {
            grid_p.push(grid_0.clone());
        }

        PocketDimension {
            grid_n,
            grid_0,
            grid_p,
        }
    }

    fn next(&self) -> PocketDimension {
        let mut next = self.grow();
        for (x, y, z) in next.cells() {
            let cell = self.alive((x, y, z));
            let neighbours = neighbours((x, y, z))
                .filter(|(u, v, w)| self.alive((*u, *v, *w)))
                .count();

            if cell {
                if neighbours == 2 || neighbours == 3 {
                    next[(x, y, z)] = true;
                } else {
                    next[(x, y, z)] = false;
                }
            } else if neighbours == 3 {
                next[(x, y, z)] = true;
            }
        }

        next
    }
}

impl std::ops::Index<(isize, isize, isize)> for PocketDimension {
    type Output = bool;

    fn index(&self, (x, y, z): (isize, isize, isize)) -> &Self::Output {
        if z == 0 {
            &self.grid_0[(x, y)]
        } else if z > 0 {
            let i = z - 1;
            assert!(i >= 0 && i < self.grid_p.len() as isize);
            &self.grid_p[i as usize][(x, y)]
        } else {
            let i = -z - 1;
            assert!(i >= 0 && i < self.grid_n.len() as isize);
            &self.grid_n[i as usize][(x, y)]
        }
    }
}
impl std::ops::IndexMut<(isize, isize, isize)> for PocketDimension {
    fn index_mut(&mut self, (x, y, z): (isize, isize, isize)) -> &mut Self::Output {
        if z == 0 {
            &mut self.grid_0[(x, y)]
        } else if z > 0 {
            let i = z - 1;
            assert!(i >= 0 && i < self.grid_p.len() as isize);
            &mut self.grid_p[i as usize][(x, y)]
        } else {
            let i = -z - 1;
            assert!(i >= 0 && i < self.grid_n.len() as isize);
            &mut self.grid_n[i as usize][(x, y)]
        }
    }
}

fn neighbours((x, y, z): (isize, isize, isize)) -> impl Iterator<Item = (isize, isize, isize)> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .map(|((dx, dy), dz)| (dx, dy, dz))
        .filter(|(dx, dy, dz)| *dx != 0 || *dy != 0 || *dz != 0)
        .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz))
}

fn main() -> Result<()> {
    let grid_0 = read_input()?;

    println!("part 1: {}", part1(&grid_0));
    println!("part 2: {}", part2(&grid_0));

    Ok(())
}

fn read_input() -> Result<Grid> {
    let mut grid = None;
    for (y, line) in BufReader::new(File::open("input")?).lines().enumerate() {
        let line = line?;
        let grid = match grid {
            None => {
                grid = Some(Grid::new(line.len() as isize));
                grid.as_mut().unwrap()
            }
            Some(ref mut g) => g,
        };

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid[(x as isize, y as isize)] = true;
            }
        }
    }
    Ok(grid.unwrap())
}

fn part1(grid_0: &Grid) -> usize {
    let mut dim = PocketDimension {
        grid_n: vec![],
        grid_p: vec![],
        grid_0: grid_0.clone(),
    };

    for _ in 0..6 {
        dim = dim.next();
    }

    dim.cells().filter(|(x, y, z)| dim[(*x, *y, *z)]).count()
}

struct HyperDimension {
    min_w: isize,
    ps: Vec<PocketDimension>,
}

impl HyperDimension {
    fn alive(&self, (x, y, z, w): (isize, isize, isize, isize)) -> bool {
        let w = w - self.min_w;
        if w < 0 || w >= self.ps.len() as isize {
            false
        } else {
            self.ps[w as usize].alive((x, y, z))
        }
    }

    fn cells(&self) -> impl Iterator<Item = (isize, isize, isize, isize)> {
        let min_w = self.min_w;
        let max_w = self.min_w + self.ps.len() as isize - 1;

        let min_x = self.ps[0].grid_0.left();
        let max_x = self.ps[0].grid_0.right();

        let min_y = self.ps[0].grid_0.top();
        let max_y = self.ps[0].grid_0.bottom();

        let min_z = -1 * self.ps[0].grid_n.len() as isize;
        let max_z = self.ps[0].grid_p.len() as isize;

        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .cartesian_product(min_z..=max_z)
            .cartesian_product(min_w..=max_w)
            .map(|(((x, y), z), w)| (x, y, z, w))
    }

    fn next(&self) -> HyperDimension {
        let mut next = {
            let p = self.ps[0].grow();

            let mut ps = Vec::with_capacity(self.ps.len() + 2);
            while ps.len() < self.ps.len() + 2 {
                ps.push(p.clone());
            }

            HyperDimension {
                min_w: self.min_w - 1,
                ps,
            }
        };

        for (x, y, z, w) in next.cells() {
            let cell = self.alive((x, y, z, w));
            let hyper_neighbours = hyper_neighbours((x, y, z, w))
                .filter(|(a, b, c, d)| self.alive((*a, *b, *c, *d)))
                .count();

            if cell {
                if hyper_neighbours == 2 || hyper_neighbours == 3 {
                    next[(x, y, z, w)] = true;
                } else {
                    next[(x, y, z, w)] = false;
                }
            } else if hyper_neighbours == 3 {
                next[(x, y, z, w)] = true;
            }
        }

        next
    }
}

impl std::ops::Index<(isize, isize, isize, isize)> for HyperDimension {
    type Output = bool;
    fn index(&self, (x, y, z, w): (isize, isize, isize, isize)) -> &Self::Output {
        let w = w - self.min_w;
        assert!(w >= 0 && w < self.ps.len() as isize);
        &self.ps[w as usize][(x, y, z)]
    }
}

impl std::ops::IndexMut<(isize, isize, isize, isize)> for HyperDimension {
    fn index_mut(&mut self, (x, y, z, w): (isize, isize, isize, isize)) -> &mut Self::Output {
        let w = w - self.min_w;
        assert!(w >= 0 && w < self.ps.len() as isize);
        &mut self.ps[w as usize][(x, y, z)]
    }
}

fn hyper_neighbours(
    (x, y, z, w): (isize, isize, isize, isize),
) -> impl Iterator<Item = (isize, isize, isize, isize)> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .map(|(((dx, dy), dz), dw)| (dx, dy, dz, dw))
        .filter(|(dx, dy, dz, dw)| *dx != 0 || *dy != 0 || *dz != 0 || *dw != 0)
        .map(move |(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
}

fn part2(grid_0: &Grid) -> usize {
    let mut hyper_dim = HyperDimension {
        min_w: 0,
        ps: vec![PocketDimension {
            grid_n: vec![],
            grid_p: vec![],
            grid_0: grid_0.clone(),
        }],
    };

    for _ in 0..6 {
        hyper_dim = hyper_dim.next();
    }

    hyper_dim
        .cells()
        .filter(|(x, y, z, w)| hyper_dim[(*x, *y, *z, *w)])
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbours() {
        assert_eq!(neighbours((0, 0, 0)).count(), 26);
        assert_eq!(hyper_neighbours((0, 0, 0, 0)).count(), 80);
    }
}
