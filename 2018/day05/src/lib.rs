use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Polarity {
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Unit {
    pub value: char,
    pub polarity: Polarity,
}

impl Unit {
    fn new(c: char) -> Self {
        assert!(c.is_ascii());

        Unit {
            value: c.to_ascii_lowercase(),
            polarity: match c.is_ascii_uppercase() {
                true => Polarity::Positive,
                false => Polarity::Negative,
            },
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Io {
        cause: io::Error,
        context: &'static str,
    },
    InvalidUnit(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io { cause, context } => write!(f, "{}: {}", context, cause),
            Error::InvalidUnit(c) => write!(f, "invalid unit: {}", c),
        }
    }
}

type Result<T> = result::Result<T, Error>;

pub fn read_input() -> Result<Vec<Unit>> {
    let mut f = File::open("input").map_err(|e| Error::Io {
        cause: e,
        context: "could not open input file",
    })?;

    let buf = {
        let mut buf = String::new();
        f.read_to_string(&mut buf).map_err(|e| Error::Io {
            cause: e,
            context: "could not read input file",
        })?;

        buf
    };

    let mut units = Vec::with_capacity(buf.len() - 1);

    for c in buf.chars() {
        match c {
            '\n' => continue,
            c if !c.is_ascii() => return Err(Error::InvalidUnit(c)),
            c => units.push(Unit::new(c)),
        }
    }

    Ok(units)
}

pub fn react_polymer(units: &[Unit]) -> Vec<Unit> {
    let mut result = Vec::<Unit>::new();

    for unit in units {
        let len = result.len();

        if len > 0 {
            let last = &result[len - 1];

            if last.value == unit.value && last.polarity != unit.polarity {
                result.pop();
                continue;
            }
        }

        result.push(*unit);
    }

    result
}
