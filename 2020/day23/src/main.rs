use std::collections::VecDeque;
use std::env;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input: Vec<u8> = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Usage: day23-2020 [input]"))?
        .chars()
        .map(|c| {
            assert!(c.is_ascii_digit());
            c as u8 - b'0'
        })
        .collect();

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn part1(cups: &[u8]) -> usize {
    let mut cups: VecDeque<u8> = cups.iter().cloned().collect();
    let mut current = cups[0];

    for _ in 0..100 {
        let current_idx = cups.iter().cloned().position(|c| c == current).unwrap();

        let next_3 = [
            cups[(current_idx + 1) % cups.len()],
            cups[(current_idx + 2) % cups.len()],
            cups[(current_idx + 3) % cups.len()],
        ];

        cups.remove(cups.iter().cloned().position(|c| c == next_3[0]).unwrap());
        cups.remove(cups.iter().cloned().position(|c| c == next_3[1]).unwrap());
        cups.remove(cups.iter().cloned().position(|c| c == next_3[2]).unwrap());

        let dest_cup = {
            let mut dest_cup = current - 1;
            while !cups.contains(&dest_cup) {
                if dest_cup == 0 {
                    dest_cup = cups.iter().cloned().max().unwrap();
                } else {
                    dest_cup -= 1;
                }
            }

            dest_cup
        };

        let dest_cup_position = cups.iter().cloned().position(|c| c == dest_cup).unwrap();
        cups.insert(dest_cup_position + 1, next_3[0]);
        cups.insert(dest_cup_position + 2, next_3[1]);
        cups.insert(dest_cup_position + 3, next_3[2]);

        let current_idx = cups.iter().cloned().position(|c| c == current).unwrap();
        current = cups[(current_idx + 1) % cups.len()];
    }

    let pos_1 = cups.iter().cloned().position(|c| c == 1).unwrap();
    let cups = Vec::from(cups);

    let mut val = 0;
    for c in cups[pos_1 + 1..].iter().cloned() {
        val *= 10;
        val += c as usize;
    }
    for c in cups[..pos_1].iter().cloned() {
        val *= 10;
        val += c as usize;
    }

    val
}

fn part2(cups: &[u8]) -> u64 {
    // For a cup N, next_positions[N-1] tells you the next cup.

    // We intentionally use u64 instead of usize here as to not mix indices and cups.
    let mut next_cups = vec![0u64; 1_000_000];

    fn next_cup(cup: u64, next_cups: &[u64]) -> u64 {
        assert!(cup != 0);
        next_cups[cup as usize - 1]
    }
    fn next_cup_mut(cup: u64, next_cups: &mut [u64]) -> &mut u64 {
        assert!(cup != 0);
        &mut next_cups[cup as usize - 1]
    }

    {
        for prev_next in cups.windows(2) {
            next_cups[prev_next[0] as usize - 1] = prev_next[1] as u64;
        }

        let last = cups[cups.len() - 1] as usize;
        next_cups[last - 1] = cups.len() as u64 + 1;

        for (i, p) in next_cups.iter_mut().enumerate().skip(cups.len()) {
            *p = i as u64 + 2;
        }
        next_cups[1_000_000 - 1] = cups[0] as u64;
    }

    assert!(next_cups.iter().all(|&n| n != 0));

    let mut current_cup = cups[0] as u64;

    for _ in 0..10_000_000 {
        let removed_1 = next_cup(current_cup, &next_cups);
        let removed_2 = next_cup(removed_1, &next_cups);
        let removed_3 = next_cup(removed_2, &next_cups);
        let after_removed = next_cup(removed_3, &next_cups);

        let removed = [removed_1, removed_2, removed_3];

        let dest_cup = {
            let mut dest_cup = current_cup - 1;
            if dest_cup == 0 {
                dest_cup = 1_000_000;
            }
            while removed.contains(&dest_cup) {
                dest_cup -= 1;
                if dest_cup == 0 {
                    dest_cup = 1_000_000;
                }
            }
            dest_cup
        };

        let after_dest = next_cup(dest_cup, &next_cups);

        *next_cup_mut(current_cup, &mut next_cups) = after_removed;
        *next_cup_mut(dest_cup, &mut next_cups) = removed_1;
        *next_cup_mut(removed_3, &mut next_cups) = after_dest;

        current_cup = next_cup(current_cup, &next_cups);
    }

    let first_after_1 = next_cup(1, &next_cups);
    let second_after_1 = next_cup(first_after_1, &next_cups);

    first_after_1 * second_after_1
}
