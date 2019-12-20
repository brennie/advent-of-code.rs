use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use derive_more::{Add, AddAssign};
use itertools::Itertools;

use petgraph::algo::dijkstra;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;

#[derive(Add, AddAssign, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

type Graph = GraphMap<Point, isize, Directed>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct State {
    point: Point,
    cost: usize,
    depth: isize,
}

impl Ord for State {
    fn cmp(&self, rhs: &State) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, rhs: &State) -> Option<Ordering> {
        Some(self.cost.cmp(&rhs.cost).reverse())
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let (graph, start, end) = read_input()?;

    {
        let d = dijkstra(&graph, start, Some(end), |_| 1);
        println!("part 1: {}", d[&end]);
    }

    {
        let mut open = VecDeque::new();
        let mut closed = HashSet::new();

        let start_state = State {
            point: start,
            depth: 0,
            cost: 0,
        };

        open.push_back(start_state);
        closed.insert((start_state.point, start_state.depth));

        while let Some(state) = open.pop_front() {
            if state.depth == 0 && state.point == end {
                println!("part 2: {}", state.cost);
                break;
            }

            for (n, neighbour, delta_depth) in graph.edges(state.point) {
                assert_eq!(n, state.point);
                assert_ne!(state.point, neighbour);

                let next_state = if *delta_depth == 0 {
                    Some(State {
                        point: neighbour,
                        depth: state.depth,
                        cost: state.cost + 1,
                    })
                } else if *delta_depth == -1 && state.depth > 0 {
                    Some(State {
                        point: neighbour,
                        depth: state.depth - 1,
                        cost: state.cost + 1,
                    })
                } else if *delta_depth == 1 && state.depth < 100 {
                    Some(State {
                        point: neighbour,
                        depth: state.depth + 1,
                        cost: state.cost + 1,
                    })
                } else {
                    None
                };

                if let Some(next_state) = next_state {
                    if !closed.contains(&(next_state.point, next_state.depth)) {
                        open.push_back(next_state);
                        closed.insert((next_state.point, next_state.depth));
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_input() -> Result<(Graph, Point, Point), Box<dyn Error>> {
    const UP: Point = Point { x: 0, y: -1 };
    const LEFT: Point = Point { x: -1, y: 0 };

    let lines: Vec<Vec<char>> = BufReader::new(File::open("input")?)
        .lines()
        .map_results(|line| line.chars().collect())
        .collect::<Result<Vec<_>, _>>()?;

    let (inner_top_left, inner_bottom_right) = find_inner(&lines);
    let labels = find_labels(&lines, inner_top_left, inner_bottom_right);

    let height = lines.len();
    let width = lines[0].len();

    let mut g = Graph::new();

    for y in 2..height - 2 {
        for x in 2..width - 2 {
            if lines[y][x] == '.' {
                let p = Point {
                    x: x as isize,
                    y: y as isize,
                };

                g.add_node(p);

                for d in &[UP, LEFT] {
                    let q = p + *d;

                    if lines[q.y as usize][q.x as usize] == '.' {
                        g.add_edge(p, q, 0);
                        g.add_edge(q, p, 0);
                    }
                }
            }
        }
    }

    for (label, points) in &labels {
        if label == "AA" || label == "ZZ" {
            continue;
        }

        g.add_edge(points[0].0, points[1].0, points[0].1);
        g.add_edge(points[1].0, points[0].0, points[1].1);
    }

    Ok((g, labels["AA"][0].0, labels["ZZ"][0].0))
}
fn find_inner(lines: &[Vec<char>]) -> (Point, Point) {
    let height = lines.len();
    let width = lines[0].len();

    assert_eq!(lines[height / 2][width / 2], ' ');

    let mut inner_left = None;
    for x in (0..width / 2).rev() {
        match lines[height / 2][x] {
            '#' | '.' => {
                inner_left = Some(x);
                break;
            }
            _ => continue,
        }
    }
    let inner_left = inner_left.unwrap();

    let mut inner_right = None;
    for x in width / 2..width {
        match lines[height / 2][x] {
            '#' | '.' => {
                inner_right = Some(x);
                break;
            }
            _ => continue,
        }
    }
    let inner_right = inner_right.unwrap();

    let mut inner_top = None;
    for y in (0..height / 2).rev() {
        match lines[y][width / 2] {
            '#' | '.' => {
                inner_top = Some(y);
                break;
            }
            _ => continue,
        }
    }
    let inner_top = inner_top.unwrap();

    let mut inner_bottom = None;
    for y in height / 2..height {
        match lines[y][width / 2] {
            '#' | '.' => {
                inner_bottom = Some(y);
                break;
            }
            _ => continue,
        }
    }
    let inner_bottom = inner_bottom.unwrap();

    (
        Point {
            x: inner_left as isize,
            y: inner_top as isize,
        },
        Point {
            x: inner_right as isize,
            y: inner_bottom as isize,
        },
    )
}

fn find_labels(
    lines: &[Vec<char>],
    inner_top_left: Point,
    inner_bottom_right: Point,
) -> HashMap<String, Vec<(Point, isize)>> {
    let height = lines.len();
    let width = lines[0].len();

    let mut labels = HashMap::<String, Vec<(Point, isize)>>::new();

    // Outer donut.
    for (x, (c1, c2)) in Iterator::zip(lines[0].iter(), lines[1].iter()).enumerate() {
        if c1.is_ascii_uppercase() && c2.is_ascii_uppercase() {
            let name = format!("{}{}", c1, c2);
            labels.entry(name).or_default().push((
                Point {
                    x: x as isize,
                    y: 2,
                },
                -1,
            ));
        }
    }

    for (x, (c1, c2)) in
        Iterator::zip(lines[height - 2].iter(), lines[height - 1].iter()).enumerate()
    {
        if c1.is_ascii_uppercase() && c2.is_ascii_uppercase() {
            let name = format!("{}{}", c1, c2);
            labels.entry(name).or_default().push((
                Point {
                    x: x as isize,
                    y: (height - 3) as isize,
                },
                -1,
            ));
        }
    }

    for (y, (c1, c2)) in
        Iterator::zip(lines.iter().map(|s| s[0]), lines.iter().map(|s| s[1])).enumerate()
    {
        if c1.is_ascii_uppercase() && c2.is_ascii_uppercase() {
            let name = format!("{}{}", c1, c2);
            labels.entry(name).or_default().push((
                Point {
                    x: 2,
                    y: y as isize,
                },
                -1,
            ));
        }
    }

    for (y, (c1, c2)) in Iterator::zip(
        lines.iter().map(|s| s[width - 2]),
        lines.iter().map(|s| s[width - 1]),
    )
    .enumerate()
    {
        if c1.is_ascii_uppercase() && c2.is_ascii_uppercase() {
            let name = format!("{}{}", c1, c2);
            labels.entry(name).or_default().push((
                Point {
                    x: (width - 3) as isize,
                    y: y as isize,
                },
                -1,
            ));
        }
    }

    // Inner donut portals.
    let left = inner_top_left.x;
    let right = inner_bottom_right.x;
    let top = inner_top_left.y;
    let bottom = inner_bottom_right.y;

    for (x, (c1, c2)) in Iterator::zip(
        lines[top as usize + 1].iter(),
        lines[top as usize + 2].iter(),
    )
    .enumerate()
    .take(right as usize)
    .skip(left as usize)
    {
        if c1.is_ascii_uppercase() && c2.is_ascii_uppercase() {
            let name = format!("{}{}", c1, c2);
            labels.entry(name).or_default().push((
                Point {
                    x: x as isize,
                    y: top,
                },
                1,
            ));
        }
    }

    for (x, (c1, c2)) in Iterator::zip(
        lines[bottom as usize - 2].iter(),
        lines[bottom as usize - 1].iter(),
    )
    .enumerate()
    .take(right as usize)
    .skip(left as usize)
    {
        if c1.is_ascii_uppercase() && c2.is_ascii_uppercase() {
            let name = format!("{}{}", c1, c2);
            labels.entry(name).or_default().push((
                Point {
                    x: x as isize,
                    y: bottom,
                },
                1,
            ));
        }
    }

    for (y, (c1, c2)) in Iterator::zip(
        lines.iter().map(|s| s[(left + 1) as usize]),
        lines.iter().map(|s| s[(left + 2) as usize]),
    )
    .enumerate()
    {
        if c1.is_ascii_uppercase() && c2.is_ascii_uppercase() {
            let name = format!("{}{}", c1, c2);
            labels.entry(name).or_default().push((
                Point {
                    x: left,
                    y: y as isize,
                },
                1,
            ));
        }
    }

    for (y, (c1, c2)) in Iterator::zip(
        lines.iter().map(|s| s[(right - 2) as usize]),
        lines.iter().map(|s| s[(right - 1) as usize]),
    )
    .enumerate()
    {
        if c1.is_ascii_uppercase() && c2.is_ascii_uppercase() {
            let name = format!("{}{}", c1, c2);
            labels.entry(name).or_default().push((
                Point {
                    x: right as isize,
                    y: y as isize,
                },
                1,
            ));
        }
    }

    labels
}
