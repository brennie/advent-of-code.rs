use std::io::prelude::*;
use std::fs::File;

static START: u32 = 273025;
static STOP: u32 = 767253;



fn main() {
    let mut results = 0;
    'part1: for i in START..=STOP {
        let digits = i.to_string().chars().map(|c| c as u8 - '0' as u8).collect::<Vec<_>>();

        let mut same = false;
        for j in 1..digits.len() {
            if digits[j] < digits[j - 1] {
                continue 'part1;
            }

            if digits[j] == digits[j - 1] {
                same = true;
            }
        }

        if same {
            results += 1;
        }
    }

    println!("part 1: {}", results);

    let mut results = 0;
    'part2: for i in START..=STOP {
        let digits = i.to_string().chars().map(|c| c as u8 - '0' as u8).collect::<Vec<_>>();

        let mut same = false;
        let mut count = 1;
        for j in 1..digits.len() {
            if digits[j] < digits[j - 1] {
                continue 'part2;
            } else if digits[j] == digits[j - 1] {
                count += 1;
            } else {
                if count == 2 {
                    same = true;
                }
                count = 1;
            }
        }
        if count == 2 {
            same = true;
        }

        if same {
            results += 1;
        }
    }
    println!("part 2: {}", results);
}
