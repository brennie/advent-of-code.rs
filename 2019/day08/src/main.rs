use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn read_input() -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut buf = String::new();
    File::open("input")?.read_to_string(&mut buf)?;

    let layers = buf
        .as_bytes()
        .chunks_exact(WIDTH * HEIGHT)
        .map(|layer| layer.iter().map(|p| p - '0' as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Ok(layers)
}

fn main() -> Result<(), Box<dyn Error>> {
    let layers = read_input()?;

    let layer = layers
        .iter()
        .map(|layer| layer.iter().filter(|p| **p == 0).collect::<Vec<_>>().len())
        .enumerate()
        .min_by(|(_, a_count), (_, b_count)| a_count.cmp(b_count))
        .unwrap()
        .0;

    let mut ones = 0;
    let mut twos = 0;
    for p in &layers[layer] {
        match p {
            1 => ones += 1,
            2 => twos += 1,
            _ => continue,
        }
    }
    println!("part 1: {:?}", ones * twos);

    println!("part 2:");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let coord = y * WIDTH + x;
            for layer in &layers {
                match layer[coord] {
                    0 => {
                        print!(" ");
                        break;
                    }
                    1 => {
                        print!("#");
                        break;
                    }
                    _ => continue,
                }
            }
        }
        println!("");
    }

    Ok(())
}
