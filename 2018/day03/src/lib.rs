use std::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    Io(io::Error, String),
    Parse(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e, s) => write!(f, "{}: {}", s, e),
            Error::Parse(e) => write!(f, "could not parse input: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    Expected(char, Option<char>),
    ExpectedClass(String, Option<char>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Expected(c, Some(recvd)) => {
                write!(f, "expected `{}', got `{}' instead", c, recvd)
            }
            ParseError::Expected(c, None) => {
                write!(f, "expected `{}', got end of input instead", c)
            }
            ParseError::ExpectedClass(s, Some(recvd)) => {
                write!(f, "expected {}, got `{}' instead", s, recvd)
            }
            ParseError::ExpectedClass(s, None) => {
                write!(f, "expected {}, got end of input instead", s)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Rect {
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
        Rect {
            left,
            top,
            width,
            height,
        }
    }
    pub fn right(&self) -> usize {
        self.left + self.width
    }

    pub fn bottom(&self) -> usize {
        self.top + self.height
    }
}

impl FromStr for Rect {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();

        expect(&mut chars, '#')?;

        parse_number(&mut chars)?;

        expect(&mut chars, ' ')?;
        expect(&mut chars, '@')?;
        expect(&mut chars, ' ')?;

        let left = parse_number(&mut chars)?;
        expect(&mut chars, ',')?;
        let top = parse_number(&mut chars)?;

        expect(&mut chars, ':')?;
        expect(&mut chars, ' ')?;

        let width = parse_number(&mut chars)?;
        expect(&mut chars, 'x')?;
        let height = parse_number(&mut chars)?;

        if let Some(c) = chars.next() {
            return Err(ParseError::ExpectedClass("end of input".into(), Some(c)));
        }

        Ok(Rect {
            top,
            left,
            width,
            height,
        })
    }
}

fn expect<I>(chars: &mut Peekable<I>, c: char) -> Result<char, ParseError>
where
    I: Iterator<Item = char>,
{
    match chars.next() {
        Some(c) => Ok(c),
        any => Err(ParseError::Expected(c, any)),
    }
}

fn parse_number<I>(chars: &mut Peekable<I>) -> Result<usize, ParseError>
where
    I: Iterator<Item = char>,
{
    let mut acc: usize = 0;
    let mut consumed = false;
    loop {
        if let Some(c) = chars.peek() {
            if let Some(digit) = c.to_digit(10) {
                acc = acc * 10 + digit as usize;
                consumed = true;
                chars.next();
            } else if consumed {
                return Ok(acc);
            } else {
                return Err(ParseError::ExpectedClass("digit".into(), Some(*c)));
            }
        } else if consumed {
            return Ok(acc);
        } else {
            return Err(ParseError::ExpectedClass("digit".into(), None));
        }
    }
}

pub fn read_claims() -> Result<Vec<Rect>, Error> {
    let mut f =
        File::open("input").map_err(|e| Error::Io(e, "Could not open input file".into()))?;
    let mut buf = String::new();

    f.read_to_string(&mut buf)
        .map_err(|e| Error::Io(e, "Could not read input file".into()))?;

    buf.lines()
        .map(|line| str::parse(line).map_err(Error::Parse))
        .collect()
}

pub fn compute_min_dimensions(claims: &[Rect]) -> (usize, usize) {
    claims
        .iter()
        .map(|r| (r.right(), r.bottom()))
        .fold((0, 0), |(max_right, max_bottom), (right, bottom)| {
            (max(max_right, right), max(max_bottom, bottom))
        })
}
