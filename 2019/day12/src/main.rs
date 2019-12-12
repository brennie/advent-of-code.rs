use num::integer::lcm;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use derive_more::AddAssign;
use regex::Regex;

#[derive(AddAssign, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Moon {
    position: Vec3,
    velocity: Vec3,
}

impl Moon {
    fn energy(&self) -> i64 {
        let u = self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        let t = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
        u * t
    }
}

fn read_input() -> Result<Vec<Moon>, Box<dyn Error>> {
    let re = Regex::new(r"<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>").unwrap();
    BufReader::new(File::open("input")?)
        .lines()
        .map(|line| {
            line.map(|line| {
                let captures = re.captures(&line).unwrap();

                Moon {
                    position: Vec3 {
                        x: str::parse(&captures[1]).unwrap(),
                        y: str::parse(&captures[2]).unwrap(),
                        z: str::parse(&captures[3]).unwrap(),
                    },
                    velocity: Default::default(),
                }
            })
            .map_err(Into::into)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let moons = read_input()?;

    {
        let mut moons = moons.clone();
        for _ in 0..1000 {
            simulate(&mut moons);
        }
        println!("part 1: {}", moons.iter().map(Moon::energy).sum::<i64>());
    }

    {
        let mut moons = moons.clone();
        let mut x_states = HashSet::<Vec<(i64, i64)>>::new();
        let mut y_states = HashSet::<Vec<(i64, i64)>>::new();
        let mut z_states = HashSet::<Vec<(i64, i64)>>::new();

        let mut x_repeat = None;
        let mut y_repeat = None;
        let mut z_repeat = None;

        let (x, y, z) = state(&moons);
        x_states.insert(x);
        y_states.insert(y);
        z_states.insert(z);

        let mut step = 0u64;
        loop {
            simulate(&mut moons);
            step += 1;

            let (x, y, z) = state(&moons);

            if x_repeat.is_none() {
                if x_states.contains(&x) {
                    x_repeat = Some(step);
                } else {
                    x_states.insert(x);
                }
            }

            if y_repeat.is_none() {
                if y_states.contains(&y) {
                    y_repeat = Some(step);
                } else {
                    y_states.insert(y);
                }
            }

            if z_repeat.is_none() {
                if z_states.contains(&z) {
                    z_repeat = Some(step);
                } else {
                    z_states.insert(z);
                }
            }

            if x_repeat.is_some() && y_repeat.is_some() && z_repeat.is_some() {
                break;
            }
        }

        let x_repeat = x_repeat.unwrap();
        let y_repeat = y_repeat.unwrap();
        let z_repeat = z_repeat.unwrap();

        println!("part 2: {}", lcm(x_repeat, lcm(y_repeat, z_repeat)));
    }

    Ok(())
}

fn state(moons: &[Moon]) -> (Vec<(i64, i64)>, Vec<(i64, i64)>, Vec<(i64, i64)>) {
    (
        moons.iter().map(|m| (m.position.x, m.velocity.x)).collect(),
        moons.iter().map(|m| (m.position.y, m.velocity.y)).collect(),
        moons.iter().map(|m| (m.position.z, m.velocity.z)).collect(),
    )
}

fn simulate(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }

            if moons[i].position.x < moons[j].position.x {
                moons[i].velocity.x += 1;
            } else if moons[i].position.x > moons[j].position.x {
                moons[i].velocity.x -= 1;
            }

            if moons[i].position.y < moons[j].position.y {
                moons[i].velocity.y += 1;
            } else if moons[i].position.y > moons[j].position.y {
                moons[i].velocity.y -= 1;
            }

            if moons[i].position.z < moons[j].position.z {
                moons[i].velocity.z += 1;
            } else if moons[i].position.z > moons[j].position.z {
                moons[i].velocity.z -= 1;
            }
        }
    }

    for moon in moons {
        moon.position += moon.velocity;
    }
}
