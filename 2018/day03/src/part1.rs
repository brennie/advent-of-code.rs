use std::process::exit;

use day03_2018::{compute_min_dimensions, read_claims, Rect};

fn compute_overlap(claims: Vec<Rect>) -> usize {
    let (width, height) = compute_min_dimensions(&claims);

    let mut map: Vec<Vec<usize>> = {
        let mut map = Vec::with_capacity(height);

        for _ in 0..height {
            map.push(vec![0; width]);
        }

        map
    };

    for claim in claims {
        for y in claim.top..claim.bottom() {
            for x in claim.left..claim.right() {
                map[y][x] += 1;
            }
        }
    }

    let count = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|count| *count >= 2)
                .map(|_| 1)
                .fold(0, std::ops::Add::add)
        }).fold(0, std::ops::Add::add);

    count
}

fn main() {
    match read_claims().map(compute_overlap) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(result) => println!("{}", result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let claims = vec![
            Rect::new(1, 3, 4, 4),
            Rect::new(3, 1, 4, 4),
            Rect::new(5, 5, 2, 2),
        ];

        assert_eq!(compute_overlap(claims), 4);
    }
}
