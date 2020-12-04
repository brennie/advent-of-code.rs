use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let passports = read_input()?;

    println!("part 1: {}", part1(&passports));
    println!("part 2: {}", part2(&passports));

    Ok(())
}

fn read_input() -> Result<Vec<HashMap<String, String>>> {
    let f = BufReader::new(File::open("input")?);

    let mut passports = Vec::new();
    let mut passport = HashMap::<String, String>::new();

    for line in f.lines() {
        let line = line?;

        if line.len() == 0 {
            passports.push(passport);
            passport = HashMap::new();
            continue;
        }

        for part in line.split(" ") {
            let idx = part.find(":").unwrap();
            let (key, val) = part.split_at(idx);
            let val = &val[1..];

            passport.insert(key.into(), val.into());
        }
    }
    passports.push(passport);

    return Ok(passports);
}

fn is_valid(h: &HashMap<String, String>) -> bool {
    return h.contains_key("byr")
        && h.contains_key("iyr")
        && h.contains_key("eyr")
        && h.contains_key("hgt")
        && h.contains_key("hcl")
        && h.contains_key("ecl")
        && h.contains_key("pid");
}

fn part1(p: &[HashMap<String, String>]) -> usize {
    p.iter().filter(|h| is_valid(h)).map(|_| 1usize).sum()
}

fn part2(p: &[HashMap<String, String>]) -> usize {
    p.iter().filter(|h| is_valid2(h)).map(|_| 1usize).sum()
}

fn is_valid2(h: &HashMap<String, String>) -> bool {
    const ECL: &'static [&'static str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    if !is_valid(h) {
        return false;
    }

    match h.get("byr").unwrap().parse::<usize>() {
        Ok(byr) => {
            if byr < 1920 || byr > 2002 {
                return false;
            }
        }
        Err(_) => return false,
    }

    match h.get("iyr").unwrap().parse::<usize>() {
        Ok(iyr) => {
            if iyr < 2010 || iyr > 2020 {
                return false;
            }
        }
        Err(_) => return false,
    }

    match h.get("eyr").unwrap().parse::<usize>() {
        Ok(eyr) => {
            if eyr < 2020 || eyr > 2030 {
                return false;
            }
        }
        Err(_) => return false,
    }

    let hgt = h.get("hgt").unwrap();
    if hgt.ends_with("in") {
        match hgt[..hgt.len() - 2].parse::<usize>() {
            Ok(hgt) => {
                if hgt < 59 || hgt > 76 {
                    return false;
                }
            }
            Err(_) => return false,
        }
    } else if hgt.ends_with("cm") {
        match hgt[..hgt.len() - 2].parse::<usize>() {
            Ok(hgt) => {
                if hgt < 150 || hgt > 193 {
                    return false;
                }
            }
            Err(_) => return false,
        }
    } else {
        return false;
    }

    let hcl = h.get("hcl").unwrap();
    if !hcl.starts_with('#') || !hcl.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
        return false;
    }

    let ecl = h.get("ecl").unwrap();
    if !ECL.contains(&ecl.as_ref()) {
        return false;
    }

    let pid = h.get("pid").unwrap();
    if pid.len() != 9 || !pid.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    return true;
}
