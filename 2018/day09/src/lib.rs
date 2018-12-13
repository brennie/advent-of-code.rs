use std::ops::Rem;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    pub player_count: usize,
    pub highest_marble: u32,
}

pub struct Marble(u32);

pub fn run_game(options: Options) -> u32 {
    let mut circle =
        Vec::with_capacity((options.highest_marble - options.highest_marble % 23 + 1) as usize);
    let mut scores = vec![0; options.player_count];

    let player_iter = (0..options.player_count).cycle();
    let marbles_iter = 1..=options.highest_marble;

    circle.push(0);

    let mut position = 0;

    for (player, marble) in player_iter.zip(marbles_iter) {
        if marble % 23 == 0 {
            position = modulo(position as isize - 7, circle.len());

            scores[player as usize] += marble + circle.remove(position as usize);
        } else {
            position = (position + 1) % circle.len() + 1;

            if position == circle.len() {
                circle.push(marble);
            } else if position < circle.len() {
                circle.insert(position as usize, marble);
            } else {
                unreachable!()
            }
        }
    }

    *scores.iter().max().unwrap()
}

fn modulo(a: isize, b: usize) -> usize {
    let b = b as isize;
    let r = a % b;

    if r < 0 {
        (r + b) as usize
    } else {
        r as usize
    }
}
