use std::cmp::{min, max};
use std::str::FromStr;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

use derive_more::{Display, Add};

#[derive(Add, Clone, Copy, Debug, Default)]
struct Vec2D {
    x: i32,
    y: i32,
}

impl Vec2D {
    pub fn len(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
use std::num::ParseIntError;

#[derive(Debug, Display)]
enum ParseVecError {
    #[display(fmt = "Expected non-empty string.")]
    Empty,

    #[display(fmt = "Invalid direction `{}'; expected `U', `D', `L', or `R'.", _0)]
    Direction(char),

    #[display(fmt = "{}", _0)]
    Num(ParseIntError),
}

impl Error for ParseVecError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Num(ref e) => Some(e),
            _ => None,
        }
    }
}
impl FromStr for Vec2D {
    type Err = ParseVecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let mut vec = Vec2D::default();

        let (field, mult) = match chars.next() {
            Some('U') => (&mut vec.y, 1),
            Some('D') => (&mut vec.y, -1),
            Some('L') => (&mut vec.x, -1),
            Some('R') => (&mut vec.x, 1),
            Some(c) => return Err(ParseVecError::Direction(c)),
            None => return Err(ParseVecError::Empty),
        };

        *field = mult * chars.as_str().parse::<u32>().map_err(ParseVecError::Num)? as i32;

        Ok(vec)
    }
}

type Wire = Vec<Vec2D>;

fn read_input() -> Result<(Wire, Wire), Box<dyn Error>> {
    let mut buf = String::new();
    File::open("input")?.read_to_string(&mut buf)?;

    let mut wires = buf.lines()
        .map(|s| s.split(',').map(str::parse::<Vec2D>).collect::<Result<Vec<_>, _>>())
        .collect::<Result<Vec<Vec<_>>, _>>()?;

    Ok((
        std::mem::replace(&mut wires[0], vec![]),
        std::mem::replace(&mut wires[1], vec![]),
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (w1, w2) = read_input()?;

    println!("w1: {:?}", w1);

    let s1 = segments(&w1);
    let s2 = segments(&w2);

    let intersections = intersections(&s1, &s2);
    println!("s1: {:?}", s1);
    println!("intersection: {:?}", intersections);
    let intersection = intersections.iter()
        .map(|(v, _)| v)
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    println!("part 1: {:?}", intersection);

    let fewest_steps = intersections.iter()
        .map(|(_, steps)| steps)
        .min().unwrap();

    println!("part 2: {:?}", fewest_steps);
    Ok(())
}

#[derive(Debug, Default)]
struct Segment {
    start: Vec2D,
    stop: Vec2D,
    distance: i32,
}

fn segments(w: &Wire) -> Vec<Segment> {
    w
        .iter()
        .scan((Vec2D::default(), 0), |st, &v| {
            let start = st.0;
            let stop = start + v;

            let distance = st.1;
            let next_distance = distance + v.len();

            *st = (stop, next_distance);

            Some(Segment {
                start,
                stop,
                distance,
            })
        })
        .collect()
}

fn intersections(s1: &[Segment], s2: &[Segment]) -> Vec<(Vec2D, i32)> {
    let mut results = vec![];
    for us in s1 {
        for vs in s2 {
            if us.start.x == us.stop.x && vs.start.y == vs.stop.y {
                let x = us.start.x;
                let y = vs.start.y;

                let min_x = min(vs.start.x, vs.stop.x);
                let max_x = max(vs.start.x, vs.stop.x);

                let min_y = min(us.start.y, us.stop.y);
                let max_y = max(us.start.y, us.stop.y);

                if min_x < x && x < max_x && min_y < y && y < max_y {
                    if x == 0{
                        eprintln!("zero");
                    }

                    results.push((
                        Vec2D {
                            x,
                            y,
                        },
                        us.distance + vs.distance + (x - min_x) + (y - min_y),
                    ));
                }
            } else if us.start.y == us.stop.y && vs.start.x == vs.stop.x {
                let x = vs.start.x;
                let y = us.start.y;

                let min_x = min(us.start.x, us.stop.x);
                let max_x = max(us.start.x, us.stop.x);

                let min_y = min(vs.start.y, vs.stop.y);
                let max_y = max(vs.start.y, vs.stop.y);

                if min_x < x && x < max_x && min_y < y && y < max_y {
                    if x == 0{
                        eprintln!("x: {} <= {} <= {}", min_x, x, max_x);
                    }
                    results.push((
                        Vec2D {
                            x,
                            y,
                        },
                        us.distance + vs.distance + (x - min_x) + (y - min_y)
                    ));
                }
            }
        }
    }

    results
}
