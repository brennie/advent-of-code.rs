use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::result;

use combine::stream::state::State;
use combine::Parser;

use self::parser::{points, ParseError};

#[derive(Debug)]
pub enum Error {
    Io {
        cause: io::Error,
        context: &'static str,
    },
    Parse(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io { cause, context } => write!(f, "{}: {}", context, cause),
            Error::Parse(e) => write!(f, "{}", e.0),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> u32 {
        ((self.y - other.y).abs() + (self.x - other.x).abs()) as u32
    }
}

pub fn read_coords() -> Result<Vec<Point>> {
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

    let result = points()
        .easy_parse(State::new(&*buf))
        .map(|(output, _)| output)
        .map_err(|e| Error::Parse(e.into()));

    result
}

mod parser {
    use combine::easy::{self, Errors};
    use combine::parser::char::{digit, string};
    use combine::parser::item::token;
    use combine::parser::repeat::many1;
    use combine::stream::state::{SourcePosition, State};
    use combine::{eof, Parser, Stream};

    use super::Point;

    // We are wrapping Errors<..> instead of making a type alias so we can impl
    // From for it.
    //
    // Our input type is `State<&str, _>`, but `Record` does not have a handle
    // to the underyling string, so `impl FromStr for Record` cannot contain the
    // lifetime. Therefore, we re-map errors to allocate `String`s instead.
    #[derive(Debug)]
    pub struct ParseError(pub Errors<char, String, SourcePosition>);
    type UpstreamParseError<'a> = easy::ParseError<State<&'a str, SourcePosition>>;

    impl<'a> From<UpstreamParseError<'a>> for ParseError {
        fn from(err: UpstreamParseError<'a>) -> Self {
            ParseError(Errors {
                errors: err
                    .errors
                    .into_iter()
                    .map(|e| e.map_range(String::from))
                    .collect(),
                position: err.position,
            })
        }
    }

    fn point<I>() -> impl Parser<Input = I, Output = Point>
    where
        I: Stream<Item = char, Error = easy::ParseError<I>>,
    {
        let number = || {
            many1::<Vec<u32>, _>(digit().map(|c| c.to_digit(10).unwrap()))
                .map(|digits| digits.into_iter().fold(0, |acc, i| acc * 10 + i))
        };

        (number().skip(string(", ")), number()).map(|(x, y)| Point::new(x as i32, y as i32))
    }

    pub fn points<I>() -> impl Parser<Input = I, Output = Vec<Point>>
    where
        I: Stream<Item = char, Error = easy::ParseError<I>>,
    {
        many1::<Vec<_>, _>(point().skip(token('\n'))).skip(eof())
    }
}
