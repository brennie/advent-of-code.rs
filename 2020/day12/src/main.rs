use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let input = read_input()?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn turn_left_90(&mut self) -> &mut Self {
        match self {
            Facing::North => *self = Facing::West,
            Facing::West => *self = Facing::South,
            Facing::South => *self = Facing::East,
            Facing::East => *self = Facing::North,
        }

        self
    }

    fn turn_right_90(&mut self) -> &mut Self {
        match self {
            Facing::North => *self = Facing::East,
            Facing::East => *self = Facing::South,
            Facing::South => *self = Facing::West,
            Facing::West => *self = Facing::North,
        }

        self
    }
}

struct Point {
    x: isize,
    y: isize,
}

type Direction = (char, isize);

fn read_input() -> Result<Vec<Direction>> {
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map_err(Into::into)
                .map(|s| (s.chars().nth(0).unwrap(), s[1..].parse().unwrap()))
        })
        .collect()
}

fn part1(input: &[Direction]) -> isize {
    let mut facing = Facing::East;
    let mut pos = Point { x: 0, y: 0 };

    for (c, dist) in input {
        match c {
            'N' => {
                pos.y += dist;
            }
            'S' => {
                pos.y -= dist;
            }
            'E' => {
                pos.x += dist;
            }
            'W' => {
                pos.x -= dist;
            }
            'L' => match dist {
                90 => {
                    facing.turn_left_90();
                }
                180 => {
                    facing.turn_left_90().turn_left_90();
                }
                270 => {
                    facing.turn_right_90();
                }
                360 => {}
                _ => panic!(),
            },
            'R' => match dist {
                90 => {
                    facing.turn_right_90();
                }
                180 => {
                    facing.turn_right_90().turn_right_90();
                }
                270 => {
                    facing.turn_left_90();
                }
                360 => {}
                _ => panic!(),
            },
            'F' => match facing {
                Facing::North => pos.y += dist,
                Facing::South => pos.y -= dist,
                Facing::East => pos.x += dist,
                Facing::West => pos.x -= dist,
            },
            _ => panic!(),
        }
    }

    pos.x.abs() + pos.y.abs()
}

fn part2(input: &[Direction]) -> isize {
    let mut waypoint_offset = Point { x: 10, y: 1 };
    let mut ship = Point { x: 0, y: 0 };

    for (c, dist) in input {
        match c {
            'N' => {
                waypoint_offset.y += dist;
            }
            'S' => {
                waypoint_offset.y -= dist;
            }
            'E' => {
                waypoint_offset.x += dist;
            }
            'W' => {
                waypoint_offset.x -= dist;
            }
            'F' => {
                ship.x += dist * waypoint_offset.x;
                ship.y += dist * waypoint_offset.y;
            }
            'R' => match dist {
                90 => {
                    waypoint_offset = Point {
                        x: waypoint_offset.y,
                        y: -waypoint_offset.x,
                    };
                }
                180 => {
                    waypoint_offset = Point {
                        x: -waypoint_offset.x,
                        y: -waypoint_offset.y,
                    };
                }
                270 => {
                    waypoint_offset = Point {
                        x: -waypoint_offset.y,
                        y: waypoint_offset.x,
                    };
                }
                360 => {}
                _ => panic!(),
            },
            'L' => match dist {
                90 => {
                    waypoint_offset = Point {
                        x: -waypoint_offset.y,
                        y: waypoint_offset.x,
                    };
                }
                180 => {
                    waypoint_offset = Point {
                        x: -waypoint_offset.x,
                        y: -waypoint_offset.y,
                    };
                }
                270 => {
                    waypoint_offset = Point {
                        x: waypoint_offset.y,
                        y: -waypoint_offset.x,
                    };
                }
                360 => {}
                _ => panic!(),
            },

            _ => panic!(),
        }
    }

    ship.x.abs() + ship.y.abs()
}
