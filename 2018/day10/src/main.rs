use std::cmp::{max, min};
use std::collections::HashSet;
use std::mem::swap;

use day10::{read_stars, Star, Vec2};

use failure::Error;

fn main() -> Result<(), Error> {
    let (t, sky) = find_minimum_area(read_stars()?);

    println!("t = {}", t);
    print_sky(&sky);

    Ok(())
}

fn find_minimum_area(mut sky: Vec<Star>) -> (u32, Vec<Star>) {
    let mut next_sky = Vec::with_capacity(sky.len());
    let mut area = compute_area(&sky);
    let mut t = 0;

    loop {
        next_sky.extend(sky.iter().map(|star| star.next()));
        let next_area = compute_area(&next_sky);

        if next_area < area {
            swap(&mut next_sky, &mut sky);
            next_sky.clear();

            area = next_area;
            t += 1;
        } else {
            break;
        }
    }

    (t, sky)
}

fn compute_corners(sky: &[Star]) -> (Vec2, Vec2) {
    sky.iter().map(|star| star.position).fold(
        (Vec2::new(0, 0), Vec2::new(0, 0)),
        |(mut tl, mut br), position| {
            tl.x = min(tl.x, position.x);
            tl.y = min(tl.y, position.y);

            br.x = max(br.x, position.x);
            br.y = max(br.y, position.y);

            (tl, br)
        },
    )
}

fn compute_area(sky: &[Star]) -> u64 {
    let (top_left, bottom_right) = compute_corners(sky);

    (bottom_right.y - top_left.y).abs() as u64 * (bottom_right.x - top_left.x).abs() as u64
}

fn print_sky(sky: &[Star]) {
    let offset = sky
        .iter()
        .map(|star| star.position)
        .fold(None, |offset: Option<Vec2>, position| {
            if let Some(mut offset) = offset {
                offset.x = min(offset.x, position.x);
                offset.y = min(offset.y, position.y);

                Some(offset)
            } else {
                Some(position)
            }
        })
        .expect("No stars?");

    let (top_left, bottom_right) = compute_corners(&sky);
    let width = bottom_right.x - top_left.x - offset.x;
    let height = bottom_right.y - top_left.y - offset.y;

    let points = sky
        .iter()
        .map(|star| star.position - offset)
        .collect::<HashSet<Vec2>>();

    for y in 0..=height {
        for x in 0..=width {
            if points.contains(&Vec2::new(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
