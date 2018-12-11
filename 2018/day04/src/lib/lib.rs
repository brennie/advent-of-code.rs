pub mod state_machine;

use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

use chrono::naive::NaiveDateTime;
use combine::stream::state::State;
use combine::Parser;

pub use self::parser::ParseError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error, String),
    Parse(ParseError, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e, s) => write!(f, "{}: {}", s, e),
            Error::Parse(e, n) => write!(f, "parse error on line {}: {}", n, e),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordKind {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Record {
    pub timestamp: NaiveDateTime,
    pub kind: RecordKind,
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl FromStr for Record {
    type Err = parser::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::record()
            .easy_parse(State::new(s))
            .map_err(Into::into)
            .map(|(output, _)| output)
    }
}

pub fn read_records() -> Result<Vec<Record>, Error> {
    let mut f =
        File::open("input").map_err(|e| Error::Io(e, "Could not open input file".into()))?;
    let mut buf = String::new();

    f.read_to_string(&mut buf)
        .map_err(|e| Error::Io(e, "Could not read input file".into()))?;

    let records = buf
        .lines()
        .enumerate()
        .map(|(n, s)| str::parse(s).map_err(|e| Error::Parse(e, n)))
        .collect::<Result<_, _>>()?;

    Ok(records)
}

mod parser {
    use std::fmt;

    use combine::combinator::many1;
    use combine::easy::Errors;
    use combine::parser::char::{digit, string};
    use combine::parser::item::token;
    use combine::stream::state::{SourcePosition, State};
    use combine::{choice, easy, eof, Parser, Stream};

    use chrono::naive::{NaiveDate, NaiveDateTime};

    use super::{Record, RecordKind};

    // We are wrapping Errors<..> instead of making a type alias so we can impl
    // From for it.
    //
    // Our input type is `State<&str, _>`, but `Record` does not have a handle
    // to the underyling string, so `impl FromStr for Record` cannot contain the
    // lifetime. Therefore, we re-map errors to allocate `String`s instead.
    #[derive(Debug)]
    pub struct ParseError(Errors<char, String, SourcePosition>);
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

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    fn decimal_digit<I>() -> impl Parser<Input = I, Output = u32>
    where
        I: Stream<Item = char, Error = easy::ParseError<I>>,
    {
        digit().map(|d| d.to_digit(10).unwrap())
    }

    pub fn datetime<I>() -> impl Parser<Input = I, Output = NaiveDateTime>
    where
        I: Stream<Item = char, Error = easy::ParseError<I>>,
    {
        let double_digit = || (decimal_digit(), decimal_digit()).map(|(a, b)| a * 10 + b);
        let year = || {
            (
                decimal_digit(),
                decimal_digit(),
                decimal_digit(),
                decimal_digit(),
            )
                .map(|(a, b, c, d)| a * 1000 + b * 100 + c * 10 + d)
        };

        (
            year().skip(token('-')),
            double_digit().skip(token('-')),
            double_digit().skip(token(' ')),
            double_digit().skip(token(':')),
            double_digit(),
        )
            .map(|(year, month, day, hour, minute)| {
                NaiveDate::from_ymd(year as i32, month, day).and_hms(hour, minute, 0)
            })
    }

    pub fn record_kind<I>() -> impl Parser<Input = I, Output = RecordKind>
    where
        I: Stream<Item = char, Error = easy::ParseError<I>>,
    {
        choice((
            string("Guard #")
                .with(many1::<Vec<_>, _>(decimal_digit()))
                .skip(string(" begins shift"))
                .map(|digits| digits.into_iter().fold(0, |hi, lo| hi * 10 + lo))
                .map(|n| RecordKind::BeginShift(n)),
            string("falls asleep").map(|_| RecordKind::FallAsleep),
            string("wakes up").map(|_| RecordKind::WakeUp),
        ))
    }

    pub fn record<I>() -> impl Parser<Input = I, Output = Record>
    where
        I: Stream<Item = char, Error = easy::ParseError<I>>,
    {
        (
            token('[').with(datetime()).skip(string("] ")),
            record_kind(),
        )
            .skip(eof())
            .map(|(timestamp, kind)| Record { timestamp, kind })
    }
}
