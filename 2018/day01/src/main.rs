use std::io::{self, Read};
use std::fs::File;
use std::fmt;
use std::num::ParseIntError;
use std::process::exit;

#[derive(Debug)]
enum Error {
    Io(io::Error, String),
    Parse(ParseIntError, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e, s) => write!(f, "{}: {}", s, e),
            Error::Parse(e, s) => write!(f, "Could not parse `{}': {}", s, e),
        }
    }
}

fn run() -> Result<i32, Error> {
    let mut f = File::open("input")
        .map_err(|e| Error::Io(e, "Could not open input".into()))?;

    let mut buf = String::new();

    f.read_to_string(&mut buf)
        .map_err(|e| Error::Io(e, "Could not read input".into()))?;

    let mut freq = 0;
    for line in buf.lines() {
        let offset = str::parse::<i32>(line)
            .map_err(|e| Error::Parse(e, line.into()))?;

        freq += offset;
    }

    Ok(freq)
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
