use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

use crate::intcode::Vm;

mod intcode;

fn main() -> Result<(), Box<dyn Error>> {
    let mem = read_input()?;

    {
        let mut sum = 0;

        for y in 0..50 {
            for x in 0..50 {
                let mut vm = Vm::new(&mem);
                sum += vm.with_input(x).with_input(y).run().unwrap();
            }
        }

        println!("part 1: {}", sum);
    }

    {
        let mut ranges: Vec<Range<usize>> = vec![];

        'y_gen: for y in 0.. {
            let mut start = if y == 0 { 0 } else { ranges[y-1].start };
            loop {
                if Vm::new(&mem).with_input(start as isize).with_input(y as isize).run().unwrap() == 1 {
                    break;
                }
                start += 1;
            };


            let mut end = start + 1;
            loop {
                if Vm::new(&mem).with_input(end as isize).with_input(y as isize).run().unwrap() == 0 {
                    break;
                }

                end += 1;
            }

            ranges.push(Range { start, end });

            if y >= 99 && (ranges[y].end - ranges[y].start) >= 100 {
                'x_search: for x in ranges[y].clone() {
                    for y in y-99..y {
                        if !ranges[y].contains(&x) || !ranges[y].contains(&(x + 99)) {
                            continue 'x_search;
                        }
                    }

                    println!("part 2: {}", x * 10000 + y - 99);
                    break 'y_gen;
                }

            }
        }
    }

    Ok(())
}

fn read_input() -> Result<Vec<isize>, Box<dyn Error>> {
    let mut buf = String::new();
    File::open("input")?.read_to_string(&mut buf)?;
    buf[..buf.len() - 1]
        .split(',')
        .map(|s| str::parse::<isize>(&s).map_err(Into::into))
        .collect()
}
