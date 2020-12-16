use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use regex::Regex;
use itertools::Itertools;

enum Instruction {
    Mask([char; 36]),
    Update(usize, usize),
}

fn main() -> Result<()> {
    let input = read_input()?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn read_input() -> Result<Vec<Instruction>> {
    const MASK_EQ: &'static str = "mask = ";
    let mem_re = Regex::new(r#"mem\[(\d+)\] = (\d+)"#).unwrap();

    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map_err(Into::into).map(|line| {
                if line.starts_with(MASK_EQ) {
                    let mut mask = ['0'; 36];
                    for (i, c) in line[MASK_EQ.len()..].chars().enumerate() {
                        mask[i] = c;
                    }
                    Instruction::Mask(mask)
                } else {
                    let cap = mem_re.captures(&line).unwrap();
                    Instruction::Update(cap[1].parse().unwrap(), cap[2].parse().unwrap())
                }
            })
        })
        .collect()
}

fn part1(input: &[Instruction]) -> usize {
    let mut mask = None;
    let mut mem = HashMap::<usize, usize>::new();

    for instruction in input {
        match instruction {
            Instruction::Mask(m) => {
                mask = Some(m);
            }
            Instruction::Update(addr, val) => {
                mem.insert(*addr, apply_mask(*val, mask.as_ref().unwrap()));
            }
        }
    }

    mem.values().sum::<usize>()
}

fn apply_mask(x: usize, mask: &[char; 36]) -> usize {
    let mut result = 0;

    let mut shift = 35;
    for mask_bit in &mask[..] {
        let bit = (x >> shift) & 1;

        shift -= 1;
        result <<= 1;

        result |= match mask_bit {
            'X' => bit,
            '1' => 1,
            '0' => 0,
            _ => panic!(),
        };
    }

    result
}

fn update_mem_floating(map: &mut HashMap<usize, usize>, addr: usize, val: usize, mask: &[char; 36]) {
    let mut addr = addr;
    for idx in mask[..].iter().positions(|c| *c == '1') {
        addr |= 1 << (35 - idx);
    }

    update_xs(map, addr, val, &mask);
}

fn update_xs(map: &mut HashMap<usize, usize>, addr: usize, val: usize, mask: &[char; 36]) {
    let mut new_mask = mask.clone();

    match &mask[..].iter().positions(|c| *c == 'X').next() {
        None => {
            map.insert(addr, val);
        },
        Some(idx) => {
            new_mask[*idx] = '0';
            let shift = 35 - *idx;

            let lo = addr & !(1 << shift);
            let hi = addr | (1 << shift);

            if new_mask.iter().find(|c| **c == 'X').is_none() {
                map.insert(lo, val);
                map.insert(hi, val);
            } else {
                update_xs(map, lo, val, &new_mask);
                update_xs(map, hi, val, &new_mask);
            }
        }
    }
}

fn part2(input: &[Instruction]) -> usize {
    let mut mask = None;
    let mut mem = HashMap::<usize, usize>::new();

    for instruction in input {
        match instruction {
            Instruction::Mask(m) => {
                mask = Some(m);
            }
            Instruction::Update(addr, val) => {
                update_mem_floating(&mut mem, *addr, *val, mask.as_ref().unwrap());
            }
        }
    }

    let mut f = File::create("rust").unwrap();
    let mut values = mem.iter().collect::<Vec<_>>();
    values.sort();

    use std::io::Write;
    for v in &values {
        writeln!(f, "{} = {}", v.0, v.1).unwrap();
    }

    mem.values().sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply_mask() {
        let mut mask: [char; 36] = ['0'; 36];
        mask.copy_from_slice("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".chars().collect::<Vec<_>>().as_ref());
        assert_eq!(apply_mask(11, &mask), 73);
    }

    #[test]
    fn test_part2() {
        let mut mem = HashMap::new();
        let mut mask: [char; 36] = ['1'; 36];

        update_mem_floating(&mut mem, 42, 100, &mask);
        assert_eq!(*mem.keys().next().unwrap(), 0b111111111111111111111111111111111111);
        assert_eq!(mem.len(), 1);

    }

    #[test]
    fn test_part2_sample() {
        let mut mem = HashMap::new();
        let mut mask: [char; 36] = ['0'; 36];
        mask.copy_from_slice("000000000000000000000000000000X1001X".chars().collect::<Vec<_>>().as_ref());
        update_mem_floating(&mut mem, 42, 100, &mask);

        assert_eq!(*mem.get(&26).unwrap(), 100);
        assert_eq!(*mem.get(&27).unwrap(), 100);
        assert_eq!(*mem.get(&58).unwrap(), 100);
        assert_eq!(*mem.get(&59).unwrap(), 100);
        assert_eq!(mem.len(), 4);

        mask.copy_from_slice("00000000000000000000000000000000X0XX".chars().collect::<Vec<_>>().as_ref());
        update_mem_floating(&mut mem, 26, 1, &mask);

        assert_eq!(*mem.get(&16).unwrap(), 1);
        assert_eq!(*mem.get(&17).unwrap(), 1);
        assert_eq!(*mem.get(&18).unwrap(), 1);
        assert_eq!(*mem.get(&19).unwrap(), 1);
        assert_eq!(*mem.get(&24).unwrap(), 1);
        assert_eq!(*mem.get(&25).unwrap(), 1);
        assert_eq!(*mem.get(&26).unwrap(), 1);
        assert_eq!(*mem.get(&27).unwrap(), 1);
        assert_eq!(*mem.get(&58).unwrap(), 100);
        assert_eq!(*mem.get(&59).unwrap(), 100);
        assert_eq!(mem.len(), 10);
    }
}
