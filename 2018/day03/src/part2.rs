use std::collections::HashSet;
use std::process::exit;

use day03::{read_claims, Rect};

fn intersects(a: &Rect, b: &Rect) -> bool {
    a.left < b.right() && a.right() > b.left && a.top < b.bottom() && a.bottom() > b.top
}

fn find_outlier(claims: Vec<Rect>) -> Option<usize> {
    let claims_with_ids = || claims.iter().zip(1..);
    let mut open_ids = (1..=claims.len()).collect::<HashSet<_>>();

    for (a, a_id) in claims_with_ids() {
        for (b, b_id) in claims_with_ids() {
            if a_id == b_id {
                continue;
            }

            if intersects(&a, &b) {
                open_ids.remove(&a_id);
                open_ids.remove(&b_id);
            }
        }
    }

    if open_ids.len() == 1 {
        open_ids.into_iter().next()
    } else {
        None
    }
}

fn main() {
    match read_claims().map(find_outlier) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(None) => {
            eprintln!("error: no (unique) solution");
            exit(1);
        }

        Ok(Some(result)) => println!("{}", result),
    }
}
