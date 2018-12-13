use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::result;

use combine::stream::state::State;
use combine::Parser;

pub use self::parser::ParseError;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Vertex(pub char);

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    from: Vertex,
    to: Vertex,
}

#[derive(Debug)]
pub struct Graph(pub HashMap<Vertex, HashSet<Vertex>>);

impl Graph {
    pub fn from_edges(edges: Vec<Edge>) -> Self {
        let g = edges
            .into_iter()
            .fold(HashMap::<Vertex, HashSet<Vertex>>::new(), |mut g, e| {
                g.entry(e.to).or_insert_with(|| HashSet::new());
                g.entry(e.from)
                    .or_insert_with(|| HashSet::new())
                    .insert(e.to);

                g
            });

        Graph(g)
    }

    pub fn incoming<'a>(&'a self, v: Vertex) -> impl Iterator<Item = Vertex> + 'a {
        self.0.iter().filter_map(move |(from, to_set)| {
            if to_set.contains(&v) {
                Some(*from)
            } else {
                None
            }
        })
    }
}

pub fn read_edges() -> Result<Vec<Edge>> {
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

    buf.lines()
        .map(|line| {
            parser::edge()
                .easy_parse(State::new(&*line))
                .map_err(|e| Error::Parse(e.into()))
                .map(|(edge, _)| edge)
        })
        .collect::<Result<_>>()
}

mod parser {
    use combine::easy::{self, Errors};
    use combine::parser::char::string;
    use combine::parser::item::satisfy;
    use combine::stream::state::{SourcePosition, State};
    use combine::{eof, Parser, Stream};

    use super::{Edge, Vertex};

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

    pub fn edge<I>() -> impl Parser<Input = I, Output = Edge>
    where
        I: Stream<Item = char, Error = easy::ParseError<I>>,
    {
        (
            string("Step ")
                .with(satisfy(char::is_alphabetic))
                .map(Vertex),
            string(" must be finished before step ")
                .with(satisfy(char::is_alphabetic))
                .map(Vertex),
        )
            .skip(string(" can begin."))
            .skip(eof())
            .map(|(from, to)| Edge { from, to })
    }
}
