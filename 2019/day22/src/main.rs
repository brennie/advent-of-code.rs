use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem::swap;

use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Cut(isize),
    DealIncrement(usize),
    DealIntoNew,
}

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = read_input()?;

    {
        let mut cards = (0..=10006).collect::<VecDeque<_>>();
        for instr in &instructions {
            match instr {
                Instruction::Cut(n) => cut(&mut cards, *n),
                Instruction::DealIncrement(n) => deal_increment(&mut cards, *n),
                Instruction::DealIntoNew => deal_new(&mut cards),
            }
        }

        let pos = cards
            .iter()
            .enumerate()
            .find(|(_, card)| **card == 2019)
            .unwrap()
            .0;

        println!("part 1: {}", pos);
    }

    {
        // Each shuffle (or reverse shuffle) can be represented as an affine
        // transformation of the form:
        //
        //     f(c) = (A * c + B) (mod M)
        //
        // Since we care about the card in position 2020 at the end, we will
        // compute the affine transformation of the reverse shuffle to see which
        // card it corresponds to in the original deck.
        //
        // If we compute `A` and `B` for a single shuffle then we can repeatedly
        // apply it to generate a general case formulate for `f^n(c)` (i.e.,
        // shuffling `n` times):
        //
        //     f^2(c) = f(f(c))              (mod M)
        //            = f(A * c + B)         (mod M)
        //            = A * (A * c + B) + B  (mod M)
        //            = A^2 * c + AB + B     (mod M)
        //
        //     f^3(c) = f(f(f(c))                   (mod M)
        //            = f(A^2 * c + AB + B)         (mod M)
        //            = A * (A^2 * C + AB + B) + B  (mod M)
        //            = A^3 * c + A^2 B + AB + B    (mod M)
        //
        //     ...
        //
        //     f^n(c) = A^n * c + A^(n-1) * B + A^(n-2) * B + ... + AB + B  (mod M)
        //            = A^n * c + B * (A^(n-1) + A^(n-2) + ... + A + 1)     (mod M)
        //            = A^n * C + B * (1 + A + A^2 .. A^{n-1})              (mod M)
        //                             ______________________
        //
        // We notice that the underlined term is the first `n` terms of a
        // geometric series. In general, this sum is given by:
        //
        //     S(n) = (A^n - 1) / (A - 1)
        //
        // However, since we are operating in `Zn/Z` and n is prime, we can find
        // the inverse of `(A - 1)` by Fermat's Little Theorem. In the general
        // case:
        //
        //     1 = a^{p - 1}               (mod p)
        //     a * a^{-1} = a * a^{p - 2}  (mod p)
        //
        // And since a and p are co-prime (because all members of Zp/Z are
        // co-prime to p), we can simplify this to:
        //
        //     a^{-1} = a^{p - 2}  (mod p)
        //
        // Thus the inverse of `(A-1)` modulo `M` is given by:
        //
        //     (A - 1)^{-1} = (A - 1)^{M - 2}  (mod M)
        //
        // Therefore our closed form of `f^n(c)` is given by:
        //
        //     f^n(c) = A^n * c + B * (A^(n-1) - 1) * (A - 1)^(M - 2)  (mod M)

        const N_CARDS: i128 = 119315717514047i128;
        const N_ITERS: i128 = 101741582076661i128;

        let mut a = 1;
        let mut b = 0;

        for instr in instructions.iter().rev() {
            match instr {
                Instruction::Cut(n) => {
                    b += *n as i128;
                }

                Instruction::DealIncrement(n) => {
                    let p = modular_exp(*n as i128, N_CARDS - 2, N_CARDS);
                    a *= p;
                    b *= p;
                }

                Instruction::DealIntoNew => {
                    b += 1;
                    a *= -1;
                    b *= -1;
                }
            }

            if a < 0 {
                a += N_CARDS;
            } else {
                a %= N_CARDS;
            }

            if b < 0 {
                b += N_CARDS;
            } else {
                b %= N_CARDS;
            }
        }

        let v = ((modular_exp(a, N_ITERS, N_CARDS) * 2020) % N_CARDS
            + ((b * (modular_exp(a, N_ITERS, N_CARDS) + N_CARDS - 1)) % N_CARDS
                * modular_exp(a - 1, N_CARDS - 2, N_CARDS))
                % N_CARDS)
            % N_CARDS;

        println!("part 2: {}", v);
    }

    Ok(())
}

fn read_input() -> Result<Vec<Instruction>, Box<dyn Error>> {
    BufReader::new(File::open("input")?)
        .lines()
        .map_results(|line| {
            if line.starts_with("cut ") {
                let rest = &line[4..];
                rest.parse::<isize>().map(Instruction::Cut).unwrap()
            } else if line.starts_with("deal with increment ") {
                let rest = &line[20..];
                rest.parse::<usize>()
                    .map(Instruction::DealIncrement)
                    .unwrap()
            } else if line == "deal into new stack" {
                Instruction::DealIntoNew
            } else {
                unimplemented!()
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn cut(cards: &mut VecDeque<u32>, n: isize) {
    let n = if n < 0 {
        cards.len() + n as usize
    } else {
        n as usize
    };

    let mut tail = cards.split_off(n);
    swap(cards, &mut tail);
    cards.append(&mut tail);
}

fn deal_new(cards: &mut VecDeque<u32>) {
    for (i, j) in Iterator::zip(0..cards.len(), (0..cards.len()).rev()) {
        if i >= j {
            break;
        }

        cards.swap(i, j);
    }
}

fn deal_increment(cards: &mut VecDeque<u32>, n: usize) {
    let mut new_deck = VecDeque::from(vec![0; cards.len()]);
    let mut idx = 0;
    for card in cards.drain(..) {
        new_deck[idx] = card;
        idx += n;
        idx %= new_deck.len();
    }

    std::mem::swap(&mut new_deck, cards);
}

fn modular_exp(b: i128, p: i128, m: i128) -> i128 {
    let mut b = if b > 0 { b } else { b + m };
    let mut p = p;

    let mut result = 1;

    while p > 0 {
        if p % 2 == 1 {
            result = (result * b) % m;
            p -= 1;
        }

        p /= 2;
        b = (b * b) % m;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        {
            let mut cards = (0..10).collect::<VecDeque<_>>();

            deal_increment(&mut cards, 7);
            deal_new(&mut cards);
            deal_new(&mut cards);

            assert_eq!(cards, &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
        }
        {
            let mut cards = (0..10).collect::<VecDeque<_>>();
            cut(&mut cards, 6);
            deal_increment(&mut cards, 7);
            deal_new(&mut cards);

            assert_eq!(cards, &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
        }
        {
            let mut cards = (0..10).collect::<VecDeque<_>>();
            deal_increment(&mut cards, 7);
            deal_increment(&mut cards, 9);
            cut(&mut cards, -2);

            assert_eq!(cards, &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
        }
    }
}
