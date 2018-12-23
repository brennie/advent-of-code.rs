use std::collections::LinkedList;
use std::mem;

use structopt::StructOpt;

fn main() {
    let opts = Options::from_args();
    println!("{}", run_game(opts));
}

#[derive(StructOpt)]
pub struct Options {
    pub player_count: usize,
    pub highest_marble: u32,
}

pub struct Marble(u32);

#[derive(Debug)]
struct Circle {
    left: LinkedList<u32>,
    right: LinkedList<u32>,
}

impl Circle {
    pub fn new() -> Self {
        Circle {
            left: LinkedList::new(),
            right: LinkedList::new(),
        }
    }

    pub fn pop(&mut self) -> Option<u32> {
        let v = self.right.pop_front();

        if self.right.len() == 0 && self.left.len() > 0 {
            self.move_left_1();
        }

        v
    }

    pub fn insert_after(&mut self, v: u32) {
        if let Some(last) = self.right.pop_front() {
            self.left.push_back(last);
        }

        self.right.push_front(v)
    }

    pub fn move_right_1(&mut self) {
        if let Some(last) = self.right.pop_front() {
            self.left.push_back(last);

            if self.right.len() == 0 {
                mem::swap(&mut self.left, &mut self.right);
            }
        }
    }

    pub fn move_left_1(&mut self) {
        if let Some(last) = self.left.pop_back() {
            self.right.push_front(last);
        }
    }

    pub fn move_left_7(&mut self) {
        let to_move;

        if self.left.len() >= 7 {
            to_move = 7;
        } else {
            to_move = 7 - self.left.len();
            self.left.append(&mut self.right);
        }

        let mut tail = self.left.split_off(self.left.len() - to_move);
        self.right = {
            tail.append(&mut self.right);
            tail
        };
    }
}

pub fn run_game(options: Options) -> u32 {
    let mut circle = Circle::new();
    let mut scores = vec![0; options.player_count];

    let player_iter = (0..options.player_count).cycle();
    let marbles_iter = 1..=options.highest_marble;

    circle.insert_after(0);

    for (player, marble) in player_iter.zip(marbles_iter) {
        if marble % 23 == 0 {
            circle.move_left_7();

            scores[player] += marble + circle.pop().unwrap();
        } else {
            circle.move_right_1();
            circle.insert_after(marble);
        }
    }

    *scores.iter().max().unwrap()
}
