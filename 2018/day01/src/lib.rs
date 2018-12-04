use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::result;

#[derive(Debug)]
pub enum Error {
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

pub type Result<T> = result::Result<T, Error>;

pub fn read_offsets() -> Result<Vec<i32>> {
    let mut f = File::open("input").map_err(|e| Error::Io(e, "Could not open input".into()))?;

    let mut buf = String::new();

    f.read_to_string(&mut buf)
        .map_err(|e| Error::Io(e, "Could not read input".into()))?;

    buf.lines()
        .map(|line| str::parse(line).map_err(|e| Error::Parse(e, line.into())))
        .collect()
}
