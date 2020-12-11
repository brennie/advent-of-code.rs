use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input()?;

    {
        let mut grid = input.clone();
        steady_state(&mut grid, adjacent_rule);
        println!("part 1: {}", grid.occupied_count());
    }
    {
        let mut grid = input.clone();
        steady_state(&mut grid, visible_rule);
        println!("part 2: {}", grid.occupied_count());
    }

    Ok(())
}

fn read_input() -> Result<Grid> {
    let initial = BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map_err(Into::into).map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Cell::Occupied,
                        'L' => Cell::Empty,
                        '.' => Cell::Floor,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Result<Vec<Vec<Cell>>>>()
        .unwrap();

    let height = initial.len();
    let width = initial[0].len();
    let mut g = Grid {
        width,
        height,
        cells: vec![Cell::Floor; width * height],
    };

    for (y, row) in initial.iter().enumerate() {
        g.cells[(y * width)..((y + 1) * width)].copy_from_slice(&row[..]);
    }

    Ok(g)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Floor,
    Occupied,
    Empty,
}

#[derive(Clone, Eq, PartialEq)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn next<Rule>(&self, rule: Rule) -> Grid
    where
        Rule: Fn(&Grid, usize, usize) -> Cell + Clone + Copy,
    {
        let mut next = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self[(y, x)];
                next[(y, x)] = rule(self, y, x);
            }
        }

        next
    }

    fn occupied_count(&self) -> usize {
        self.cells.iter().filter(|&&c| c == Cell::Occupied).count()
    }

    fn neighbours(&self, y: usize, x: usize) -> Neighbours<'_> {
        Neighbours::new(self, y, x)
    }

    fn visible(&self, y: usize, x: usize) -> Visible<'_> {
        Visible::new(self, y, x)
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y * self.width + x]
    }
}

fn steady_state<Rule>(grid: &mut Grid, rule: Rule)
where
    Rule: Fn(&Grid, usize, usize) -> Cell + Clone + Copy,
{
    loop {
        let next_grid = grid.next(rule);
        let same = next_grid == *grid;
        *grid = next_grid;

        if same {
            break;
        }
    }
}

struct Neighbours<'g> {
    grid: &'g Grid,
    y: isize,
    x: isize,
    idx: usize,
}

impl<'g> Neighbours<'g> {
    fn new(grid: &'g Grid, y: usize, x: usize) -> Self {
        Self {
            grid,
            y: y as isize,
            x: x as isize,
            idx: 0,
        }
    }
}

impl<'g> Iterator for Neighbours<'g> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        const SLOPES: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let width = self.grid.width as isize;
        let height = self.grid.height as isize;

        while self.idx < SLOPES.len() {
            let (dy, dx) = SLOPES[self.idx];
            self.idx += 1;

            let y = self.y + dy;
            let x = self.x + dx;

            if y >= 0 && y < height as isize && x >= 0 && x < width {
                return Some(self.grid[(y as usize, x as usize)]);
            }
        }

        None
    }
}

struct Visible<'g> {
    grid: &'g Grid,
    y: isize,
    x: isize,
    idx: usize,
}

impl<'g> Visible<'g> {
    fn new(grid: &'g Grid, y: usize, x: usize) -> Self {
        Self {
            grid,
            y: y as isize,
            x: x as isize,
            idx: 0,
        }
    }
}

impl<'g> Iterator for Visible<'g> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        const SLOPES: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let width = self.grid.width as isize;
        let height = self.grid.height as isize;

        while self.idx < SLOPES.len() {
            let (dy, dx) = SLOPES[self.idx];
            self.idx += 1;

            let mut y = self.y + dy;
            let mut x = self.x + dx;

            while y >= 0 && y < height && x >= 0 && x < width {
                let cell = self.grid[(y as usize, x as usize)];
                if cell != Cell::Floor {
                    return Some(cell);
                }
                y += dy;
                x += dx;
            }
        }

        None
    }
}

fn adjacent_rule(grid: &Grid, y: usize, x: usize) -> Cell {
    match grid[(y, x)] {
        Cell::Floor => Cell::Floor,
        Cell::Empty => {
            if grid.neighbours(y, x).all(|n| n != Cell::Occupied) {
                Cell::Occupied
            } else {
                Cell::Empty
            }
        }
        Cell::Occupied => {
            if grid
                .neighbours(y, x)
                .filter(|&n| n == Cell::Occupied)
                .count()
                >= 4
            {
                Cell::Empty
            } else {
                Cell::Occupied
            }
        }
    }
}

fn visible_rule(grid: &Grid, y: usize, x: usize) -> Cell {
    match grid[(y, x)] {
        Cell::Floor => Cell::Floor,
        Cell::Empty => {
            if grid.visible(y, x).filter(|&v| v == Cell::Occupied).count() == 0 {
                Cell::Occupied
            } else {
                Cell::Empty
            }
        }

        Cell::Occupied => {
            if grid.visible(y, x).filter(|&v| v == Cell::Occupied).count() >= 5 {
                Cell::Empty
            } else {
                Cell::Occupied
            }
        }
    }
}
