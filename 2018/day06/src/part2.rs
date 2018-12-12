use std::process::exit;

use day06::{read_coords, Point};

const MAX_DISTANCE: u32 = 9999;

fn main() {
    match read_coords().map(find_region_size) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(size) => println!("{}", size),
    }
}

fn find_region_size(points: Vec<Point>) -> u32 {
    let centre = {
        let (sum_x, sum_y) = points
            .iter()
            .fold((0, 0), |(sum_x, sum_y), p| (sum_x + p.x, sum_y + p.y));
        let n = points.len() as i32;

        Point::new(sum_x / n, sum_y / n)
    };

    if sum_of_distances(&centre, &points) > MAX_DISTANCE {
        return 0;
    }

    let mut area = 1;
    for offset in 1.. {
        let mut delta_area = 0;

        for x in centre.x - offset..=centre.x + offset {
            if sum_of_distances(&Point::new(x, centre.y - offset), &points) <= MAX_DISTANCE {
                delta_area += 1;
            }

            if sum_of_distances(&Point::new(x, centre.y + offset), &points) <= MAX_DISTANCE {
                delta_area += 1;
            }
        }

        for y in centre.y - offset + 1..=centre.y + offset - 1 {
            if sum_of_distances(&Point::new(centre.x - offset, y), &points) <= MAX_DISTANCE {
                delta_area += 1;
            }

            if sum_of_distances(&Point::new(centre.x + offset, y), &points) <= MAX_DISTANCE {
                delta_area += 1;
            }
        }

        if delta_area == 0 {
            break;
        }

        area += delta_area;
    }

    area
}

fn sum_of_distances(p: &Point, points: &[Point]) -> u32 {
    points.iter().map(|p2| p.distance_to(p2)).sum()
}
