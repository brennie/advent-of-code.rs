use std::cmp::max;

use structopt::StructOpt;

use day11::generate_grid;

#[derive(Debug, StructOpt)]
struct Options {
    serial: i32,
}

fn main() {
    let options = Options::from_args();
    let Square { x, y, size, .. } = find_maximum_square(options.serial);

    println!("{},{},{}", x, y, size);
}

#[derive(Debug, Eq, PartialEq)]
struct Square {
    x: usize,
    y: usize,
    size: usize,
    power: i32,
}

fn find_maximum_square(serial: i32) -> Square {
    let grid = generate_grid(serial);
    let sums = {
        let mut sums = vec![vec![0; 300]; 300];

        // for y in 1..300 {
        //     sums[y][0] += sums[y - 1][0];
        // }

        // for x in 1..300 {
        //     sums[0][x] += sums[0][x - 1];
        // }

        // for y in 1..300 {
        //     for x in 1..300 {
        //         sums[y][x] += sums[y - 1][x] + sums[y][x - 1] - sums[y - 1][x - 1];
        //     }
        // }

        for y in 0..300 {
            for x in 0..300 {
                sums[y][x] = grid[y][x]
                    + if y > 0 { sums[y - 1][x] } else { 0 }
                    + if x > 0 { sums[y][x - 1] } else { 0 }
                    - if x > 0 && y > 0 {
                        sums[y - 1][x - 1]
                    } else {
                        0
                    };
            }
        }

        sums
    };

    let total_sum = grid.iter().flat_map(|row| row.iter()).sum::<i32>();

    assert_eq!(total_sum, sums[299][299]);

    let mut best = Square {
        x: 1,
        y: 1,
        power: grid[0][0],
        size: 1,
    };

    for y in 0..300 {
        for x in 0..300 {
            let max_size = 300 - max(x, y);

            for size in 0..max_size {
                let mut power = sums[y + size][x + size];

                if x > 0 && y > 0 {
                    power += sums[y - 1][x - 1];
                }

                if x > 0 {
                    power -= sums[y + size][x - 1];
                }

                if y > 0 {
                    power -= sums[y - 1][x + size];
                }

                if power > best.power {
                    best = Square {
                        x: x + 1,
                        y: y + 1,
                        size: size + 1,
                        power,
                    };
                }
            }
        }
    }
    best
}

#[cfg(test)]
mod test {
    use super::{find_maximum_square, Square};

    #[test]
    fn test_sample() {
        assert_eq!(
            find_maximum_square(18),
            Square {
                x: 90,
                y: 269,
                size: 16,
                power: 113,
            }
        );
        assert_eq!(
            find_maximum_square(42),
            Square {
                x: 232,
                y: 251,
                size: 12,
                power: 119,
            }
        );
    }
}
