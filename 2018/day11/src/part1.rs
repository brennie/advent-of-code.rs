use std::cmp::max;

use structopt::StructOpt;

use day11::generate_grid;

#[derive(Debug, StructOpt)]
struct Options {
    serial: i32,
}

fn main() {
    let options = Options::from_args();
    let grid = generate_grid(options.serial);

    let mut max_power = None;
    let mut x_max = None;
    let mut y_max = None;

    for y in 0..297 {
        for x in 0..297 {
            let power = grid[y][x]
                + grid[y][x + 1]
                + grid[y][x + 2]
                + grid[y + 1][x]
                + grid[y + 1][x + 1]
                + grid[y + 1][x + 2]
                + grid[y + 2][x]
                + grid[y + 2][x + 1]
                + grid[y + 2][x + 2];

            if let Some(prev_max) = max_power {
                if power > prev_max {
                    max_power = Some(max(prev_max, power));
                    y_max = Some(y);
                    x_max = Some(x);
                }
            } else {
                max_power = Some(power);
            }
        }
    }

    println!("{},{}", x_max.unwrap() + 1, y_max.unwrap() + 1)
}
