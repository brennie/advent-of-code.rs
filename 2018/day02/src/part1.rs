use std::process::exit;

use day02_2018::{read_ids, Error, Result};

#[derive(Debug, Default)]
struct Freq {
    pub two_of_a_kind: bool,
    pub three_of_a_kind: bool,
}

fn count_freq(s: &str) -> Result<Freq> {
    // We are only concerned with a-z.
    let mut freq_counts = [0; 26];

    for c in s.chars() {
        let c = c.to_ascii_lowercase();

        if !c.is_ascii() {
            return Err(Error::NonAscii(c));
        } else if !c.is_ascii_alphabetic() {
            return Err(Error::NonAlpha(c));
        }

        let index = c as usize - b'a' as usize;

        freq_counts[index] += 1
    }

    let freq = freq_counts
        .into_iter()
        .fold(Default::default(), |acc, i| match i {
            2 => Freq {
                two_of_a_kind: true,
                ..acc
            },
            3 => Freq {
                three_of_a_kind: true,
                ..acc
            },
            _ => acc,
        });

    Ok(freq)
}

fn run() -> Result<u32> {
    let mut twos = 0;
    let mut threes = 0;

    for id in read_ids()? {
        let freq = count_freq(&id)?;

        twos += freq.two_of_a_kind as u32;
        threes += freq.three_of_a_kind as u32;
    }

    Ok(twos * threes)
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(result) => println!("{}", result),
    }
}
