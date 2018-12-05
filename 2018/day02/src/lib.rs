use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::result;

#[derive(Debug)]
pub enum Error {
    Io(io::Error, String),
    NonAscii(char),
    NonAlpha(char),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e, s) => write!(f, "{}: {}", s, e),
            Error::NonAscii(c) => write!(f, "char `{}' is not ASCII", c),
            Error::NonAlpha(c) => write!(f, "char `{}' is not alphabetical", c),
        }
    }
}

pub fn read_ids() -> Result<Vec<String>> {
    let mut f =
        File::open("input").map_err(|e| Error::Io(e, "Could not open input file".into()))?;
    let mut buf = String::new();

    f.read_to_string(&mut buf)
        .map_err(|e| Error::Io(e, "Could not read input file".into()))?;

    Ok(buf.lines().map(Into::into).collect())
}
