use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Sub};

use failure::{Error, ResultExt};

use self::parser::parse_star;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Star {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Star {
    pub fn next(&self) -> Self {
        Star {
            position: self.position + self.velocity,
            velocity: self.velocity,
        }
    }
}

pub fn read_stars() -> Result<Vec<Star>, Error> {
    let mut f = File::open("input").context("Could not open input file")?;
    let buf = {
        let mut buf = String::new();
        f.read_to_string(&mut buf)
            .context("Could not read input file")?;
        buf
    };

    buf.lines().map(parse_star).collect()
}

mod parser {
    use super::{Star, Vec2};

    use combine::parser::char::{digit, space, string};
    use combine::parser::choice::optional;
    use combine::parser::item::{eof, token};
    use combine::parser::repeat::{many1, skip_many};
    use combine::stream::state::State;
    use combine::{ParseError, Parser, Stream};
    use failure::format_err;

    fn vec2<I>() -> impl Parser<Input = I, Output = Vec2>
    where
        I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        let number = || {
            (
                optional(token('-')),
                many1::<String, _>(digit()).map(|digits| str::parse::<i32>(&digits).unwrap()),
            )
                .map(|(sign, n)| match (sign, n) {
                    (Some(_), n) => -n,
                    (_, n) => n,
                })
        };

        token('<')
            .skip(skip_many(space()))
            .with((number().skip(token(',')).skip(skip_many(space())), number()))
            .skip(token('>'))
            .map(|(x, y)| Vec2 { x, y })
    }

    pub fn parse_star(s: &str) -> Result<Star, failure::Error> {
        (
            string("position=").with(vec2()).skip(space()),
            string("velocity=").with(vec2()),
        )
            .skip(eof())
            .map(|(position, velocity)| Star { position, velocity })
            .easy_parse(State::new(s))
            .map_err(|e| format_err!("Could not parse `{}': {}", s, e))
            .map(|(result, _)| result)
    }
}
