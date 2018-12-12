use std::cmp::max;
use std::collections::HashMap;
use std::process::exit;

use day06::{read_coords, Point};

fn main() {
    match read_coords().map(find_area) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(area) => println!("{}", area),
    }
}

fn find_area(points: Vec<Point>) -> u32 {
    let (width, height) = points
        .iter()
        .map(|Point { x, y }| (*x as usize, *y as usize))
        .fold((0, 0), |(max_w, max_h), (x, y)| {
            (max(max_w, x), max(max_h, y))
        });

    let mut grid = vec![vec![None; width + 1]; height + 1];

    for (i, point) in points.iter().enumerate() {
        grid[point.y as usize][point.x as usize] = Some(i);
    }

    for y in 0..height {
        for x in 0..width {
            if grid[y][x].is_none() {
                grid[y][x] = nearest_to(&Point::new(x as u32, y as u32), &points);
            }
        }
    }

    let mut finite_areas = (0..points.len()).map(|i| (i, 0)).collect::<HashMap<_, _>>();

    for x in 0..width {
        if let Some(idx) = grid[0][x] {
            finite_areas.remove(&idx);
        }

        if let Some(idx) = grid[height - 1][x] {
            finite_areas.remove(&idx);
        }
    }

    for y in 1..height - 1 {
        if let Some(idx) = grid[y][0] {
            finite_areas.remove(&idx);
        }

        if let Some(idx) = grid[y][width - 1] {
            finite_areas.remove(&idx);
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(idx) = grid[y][x] {
                finite_areas.entry(idx).and_modify(|area| *area += 1);
            }
        }
    }

    finite_areas
        .into_iter()
        .map(|(_, area)| area)
        .max()
        .expect("No areas?")
}

fn nearest_to(p: &Point, points: &[Point]) -> Option<usize> {
    points
        .iter()
        .enumerate()
        .map(|(i, p2)| (p.distance_to(p2), i))
        .fold(
            HashMap::<u32, Option<usize>>::new(),
            |mut distances, (dist, i)| {
                distances
                    .entry(dist)
                    .and_modify(|index| *index = None)
                    .or_insert(Some(i));

                distances
            },
        )
        .into_iter()
        .min_by_key(|(dist, _)| *dist)
        .map(|(_, i)| i)
        .expect("No points?")
}
