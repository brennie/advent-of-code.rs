use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::num::ParseIntError;
use std::result;

#[derive(Debug)]
pub enum Error {
    Io {
        cause: io::Error,
        context: &'static str,
    },
    Parse(ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io { cause, context } => write!(f, "{}: {}", context, cause),
            Error::Parse(e) => fmt::Display::fmt(&e, f),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub meta: Vec<u32>,
}

pub fn read_tree() -> Result<Node> {
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

    let data = buf
        .trim_right()
        .split(" ")
        .map(|s| str::parse::<u32>(s).map_err(Error::Parse))
        .collect::<Result<Vec<_>>>()?;

    let (data, node) = parse_node(&data);

    assert_eq!(data.len(), 0);

    Ok(node)
}

fn parse_node<'a>(data: &'a [u32]) -> (&'a [u32], Node) {
    let num_children = data[0] as usize;
    let num_meta = data[1] as usize;

    let mut data = &data[2..];

    let mut children = Vec::with_capacity(num_children);
    for _ in 0..num_children {
        let (rest, node) = parse_node(data);

        data = rest;
        children.push(node);
    }

    let meta = data[..num_meta].iter().map(|&v| v).collect::<Vec<_>>();

    data = &data[num_meta..];

    (data, Node { children, meta })
}
