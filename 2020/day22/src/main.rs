use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let (player1, player2) = read_input()?;

    println!("part 1: {}", play(&player1, &player2));
    println!("part 2: {}", play_recursive(&player1, &player2).1);

    Ok(())
}

fn read_input() -> Result<(Vec<usize>, Vec<usize>)> {
    #[derive(Debug)]
    enum State {
        ReadPlayer(usize),
        ReadCards(usize),
    }

    let mut player1 = Vec::new();
    let mut player2 = Vec::new();
    let mut state = State::ReadPlayer(1);

    for line in BufReader::new(File::open("input")?).lines() {
        let line = line?;

        match state {
            State::ReadPlayer(n) => {
                assert_eq!(format!("Player {}:", n), line);
                state = State::ReadCards(n);
            }

            State::ReadCards(n) => {
                if line.len() == 0 {
                    if n == 2 {
                        panic!("unexpected blank line after player 2");
                    } else {
                        assert!(n == 1);
                        state = State::ReadPlayer(2)
                    }
                } else {
                    let card = line.parse()?;
                    match n {
                        1 => player1.push(card),
                        2 => player2.push(card),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    Ok((player1, player2))
}

fn play(player1: &[usize], player2: &[usize]) -> usize {
    let mut player1: VecDeque<usize> = player1.iter().cloned().collect();
    let mut player2: VecDeque<usize> = player2.iter().cloned().collect();

    while !player1.is_empty() && !player2.is_empty() {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();

        if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }

    if !player1.is_empty() {
        score(&player1)
    } else {
        score(&player2)
    }
}

enum Player {
    Player1,
    Player2,
}

fn play_recursive(player1: &[usize], player2: &[usize]) -> (Player, usize) {
    let mut player1: VecDeque<usize> = player1.iter().cloned().collect();
    let mut player2: VecDeque<usize> = player2.iter().cloned().collect();

    let mut previous_rounds: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

    loop {
        if previous_rounds.contains(&(player1.clone(), player2.clone())) || player2.is_empty() {
            return (Player::Player1, score(&player1));
        } else if player1.is_empty() {
            return (Player::Player2, score(&player2));
        }
        previous_rounds.insert((player1.clone(), player2.clone()));

        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();

        if player1.len() >= p1 && player2.len() >= p2 {
            let deck1: Vec<usize> = player1.iter().take(p1).cloned().collect();
            let deck2: Vec<usize> = player2.iter().take(p2).cloned().collect();
            match play_recursive(&deck1, &deck2).0 {
                Player::Player1 => {
                    player1.push_back(p1);
                    player1.push_back(p2);
                }
                Player::Player2 => {
                    player2.push_back(p2);
                    player2.push_back(p1);
                }
            }
        } else if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
}

fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .cloned()
        .zip(1..)
        .map(|(card, factor)| card * factor)
        .sum()
}
