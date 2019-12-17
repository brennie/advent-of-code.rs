use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;
use std::mem::swap;
use std::ops::Range;

fn main() -> Result<(), Box<dyn Error>> {
    let signal = read_input()?;

    {
        let mut input = signal.clone();
        let mut output = vec![0; input.len()];
        for _ in 0..100 {
            fft(&input, &mut output);
            swap(&mut input, &mut output);
        }

        println!("part 1: {}", string_from_digits(&input[..8]));
    }

    println!("part 2:");
    {
        let mut input = Vec::with_capacity(signal.len() * 10000);
        for _ in 0..10000 {
            input.extend(&signal);
        }

        let mut output = vec![0; input.len()];

        for i in 1..=100 {
            fft(&input, &mut output);
            println!(" {:3}/100 fft", i);

            std::mem::swap(&mut input, &mut output);
        }

        let offset = signal.iter().take(7).fold(0, |acc, n| acc * 10 + n) as usize;

        let msg = &input[offset..offset + 8];

        println!("=> {}", string_from_digits(msg));
    }
    Ok(())
}

fn read_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let mut buf = String::new();
    File::open("input")?.read_to_string(&mut buf)?;

    Ok(buf[..buf.len() - 1]
        .chars()
        .map(|c| c as i32 - '0' as i32)
        .collect::<Vec<_>>())
}

fn string_from_digits(digits: &[i32]) -> String {
    digits
        .iter()
        .map(|d| (*d as u8 + '0' as u8) as char)
        .collect()
}

struct Ranges {
    /// Position in output vector
    i: usize,

    /// Length of output vector
    len: usize,

    /// The internal counter for which ranges we generate.
    k: usize,

    /// Whether we are generating a positive range or a negative range.
    pos: bool,

    /// The next range that we will yield.
    range: Option<Range<usize>>,
}

impl Ranges {
    fn new(i: usize, len: usize) -> Ranges {
        Ranges {
            i,
            len,
            k: 0,
            pos: true,
            range: Ranges::range(i, 0, true, len),
        }
    }

    fn range(i: usize, k: usize, pos: bool, len: usize) -> Option<Range<usize>> {
        let mut range = if pos {
            ((4 * k + 1) * (i + 1) - 1)..((4 * k + 2) * (i + 1) - 1)
        } else {
            ((4 * k + 3) * (i + 1) - 1)..((4 * k + 4) * (i + 1) - 1)
        };

        if range.start >= len {
            None
        } else {
            if range.end > len {
                range.end = len;
            }
            Some(range)
        }
    }
}

impl Iterator for Ranges {
    type Item = (Range<usize>, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(range) = self.range.take() {
            let item = Some((range, if self.pos { 1 } else { -1 }));

            self.pos = !self.pos;
            if self.pos {
                self.k += 1;
            }

            self.range = Ranges::range(self.i, self.k, self.pos, self.len);

            item
        } else {
            None
        }
    }
}

fn fft(input: &[i32], output: &mut [i32]) {
    let partial_sums: Vec<_> = once(0)
        .chain(input.iter().scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        }))
        .collect();

    for i in 0..input.len() {
        let mut sum = 0;
        for (range, mult) in Ranges::new(i, input.len()) {
            let psum = partial_sums[range.end] - partial_sums[range.start];
            sum += mult * psum;
        }
        output[i] = sum.abs() % 10;
    }
}

#[cfg(test)]
mod test {
    use super::fft;
    use std::mem::swap;

    #[test]
    fn test() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output = vec![0; input.len()];

        fft(&input, &mut output);
        assert_eq!(output, [4, 8, 2, 2, 6, 1, 5, 8]);

        swap(&mut input, &mut output);
        fft(&input, &mut output);
        assert_eq!(output, [3, 4, 0, 4, 0, 4, 3, 8]);

        swap(&mut input, &mut output);
        fft(&input, &mut output);
        assert_eq!(output, [0, 3, 4, 1, 5, 5, 1, 8]);

        swap(&mut input, &mut output);
        fft(&input, &mut output);
        assert_eq!(output, [0, 1, 0, 2, 9, 4, 9, 8]);
    }
}
