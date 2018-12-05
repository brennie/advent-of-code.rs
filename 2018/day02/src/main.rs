use std::default::Default;
use std::fs::File;
use std::fmt;
use std::io::{self, Read};
use std::process::exit;
use std::result;

#[derive(Debug)]
enum Error {
    Io(io::Error, String),
    NonAscii(char),
    NonAlpha(char),
}

type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e, s) => write!(f, "{}: {}", s, e),
            Error::NonAscii(c) => write!(f, "char `{}' is not ASCII", c),
            Error::NonAlpha(c) => write!(f, "char `{}' is not alphabetical", c)
        }
    }
}

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
        .fold(Default::default(), |acc, i| {
            match i {
                2 => Freq { two_of_a_kind: true, .. acc },
                3 => Freq { three_of_a_kind: true, .. acc },
                _ => acc,
            }
        });

    Ok(freq)
}

fn run() -> Result<u32> {
    let mut f = File::open("input").map_err(|e| Error::Io(e, "Could not open input file".into()))?;
    let mut buf = String::new();

    f.read_to_string(&mut buf).map_err(|e| Error::Io(e, "Could not read input file".into()))?;

    let mut twos = 0;
    let mut threes = 0;

    for line in buf.lines() {
        let freq = count_freq(line)?;

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
