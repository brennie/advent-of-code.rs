use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input()?;

    {
        let mut dim = Dimension::from_indices(input.iter().cloned().map(|(x, y)| (x, y, 0, 0)));
        for _ in 0..6 {
            dim = dim.next_3d();
        }
        println!("part 1: {}", dim.alive_count());
    }
    {
        let mut dim = Dimension::from_indices(input.iter().cloned().map(|(x, y)| (x, y, 0, 0)));
        for _ in 0..6 {
            dim = dim.next_4d();
        }
        println!("part 2: {}", dim.alive_count());
    }

    Ok(())
}

fn read_input() -> Result<Vec<(isize, isize)>> {
    let mut coords = vec![];
    for (y, line) in BufReader::new(File::open("input")?).lines().enumerate() {
        let line = line?;

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                coords.push((x as isize, y as isize))
            }
        }
    }
    Ok(coords)
}

pub type Index = (isize, isize, isize, isize);

#[derive(Debug)]
struct Dimension {
    coords: HashSet<Index>,
    range: (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ),
}

impl Dimension {
    fn update_range(&mut self, (x, y, z, w): Index) {
        Self::update_range_impl(&mut self.range.0, x);
        Self::update_range_impl(&mut self.range.1, y);
        Self::update_range_impl(&mut self.range.2, z);
        Self::update_range_impl(&mut self.range.3, w);
    }

    fn update_range_impl(range: &mut RangeInclusive<isize>, coord: isize) {
        if coord < *range.start() {
            *range = coord..=*range.end()
        } else if coord > *range.end() {
            *range = *range.start()..=coord
        }
    }

    pub fn from_indices<I>(indices: I) -> Self
    where
        I: Iterator<Item = Index>,
    {
        let mut dim = Dimension {
            coords: HashSet::new(),
            range: (0..=0, 0..=0, 0..=0, 0..=0),
        };

        for index in indices {
            dim.set(index);
        }

        dim
    }

    pub fn set(&mut self, index: Index) {
        self.coords.insert(index);
        self.update_range(index);
    }

    pub fn neighbours(&self, (x, y, z, w): Index) -> usize {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(((dx, dy), dz), dw)| {
                (*dx != 0 || *dy != 0 || *dz != 0 || *dw != 0)
                    && self.coords.contains(&(x + *dx, y + *dy, z + *dz, w + *dw))
            })
            .count()
    }

    pub fn next_3d(&self) -> Self {
        Self::from_indices(
            Self::indices(self.grow_range())
                .filter(|(_, _, _, w)| *w == 0)
                .filter(|(x, y, z, w)| self.evolve((*x, *y, *z, *w))),
        )
    }

    pub fn next_4d(&self) -> Self {
        Self::from_indices(
            Self::indices(self.grow_range()).filter(|(x, y, z, w)| self.evolve((*x, *y, *z, *w))),
        )
    }

    pub fn alive_count(&self) -> usize {
        self.coords.len()
    }

    fn evolve(&self, coord: Index) -> bool {
        let neighbour_count = self.neighbours(coord);
        if self.coords.contains(&coord) {
            if neighbour_count == 2 || neighbour_count == 3 {
                true
            } else {
                false
            }
        } else if neighbour_count == 3 {
            true
        } else {
            false
        }
    }

    fn grow_range(
        &self,
    ) -> (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        (
            *self.range.0.start() - 1..=*self.range.0.end() + 1,
            *self.range.1.start() - 1..=*self.range.1.end() + 1,
            *self.range.2.start() - 1..=*self.range.2.end() + 1,
            *self.range.3.start() - 1..=*self.range.3.end() + 1,
        )
    }

    fn indices(
        range: (
            RangeInclusive<isize>,
            RangeInclusive<isize>,
            RangeInclusive<isize>,
            RangeInclusive<isize>,
        ),
    ) -> impl Iterator<Item = (isize, isize, isize, isize)> {
        range
            .0
            .cartesian_product(range.1)
            .cartesian_product(range.2)
            .cartesian_product(range.3)
            .map(|(((x, y), z), w)| (x, y, z, w))
    }
}
