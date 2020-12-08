use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use itertools::Itertools;
use maplit::hashmap;
use regex::Regex;

type Bags = HashMap<String, BagContents>;
type BagContents = HashMap<String, usize>;

fn main() -> Result<()> {
    let bags = read_input()?;

    println!("part 1: {}", part1(&bags));
    println!("part 2: {}", part2(&bags));

    Ok(())
}

fn read_input() -> Result<Bags> {
    let line_re = Regex::new(r#"^(.+?) bags contain (.+?).$"#).unwrap();
    let contains_re = Regex::new(r#"(\d+) (.+?) bags?(?:, )?"#).unwrap();

    let f = BufReader::new(File::open("input")?);
    let mut bags = Bags::new();
    for line in f.lines() {
        let line = line?;
        let m = line_re.captures(&line).unwrap();

        let bag = m[1].into();
        if &m[2] == "no other bags" {
            bags.insert(bag, BagContents::new());
        } else {
            let contents = contains_re
                .captures_iter(&m[2])
                .map(|cap| (String::from(&cap[2]), cap[1].parse().unwrap()))
                .collect::<BagContents>();

            bags.insert(bag, contents);
        }
    }

    Ok(bags)
}

fn part1(bags: &Bags) -> usize {
    let mut memo = HashMap::<String, bool>::new();
    let mut count = 0;

    for bag in bags.keys() {
        if contains_shiny_gold(bag, bags, &mut memo) {
            count += 1;
        }
    }

    return count;
}

fn contains_shiny_gold(bag: &str, bags: &Bags, memo: &mut HashMap<String, bool>) -> bool {
    for (sub_bag, _) in bags.get(bag).unwrap().iter() {
        let mut found = false;

        if sub_bag == "shiny gold" {
            found = true;
        } else if let Entry::Occupied(e) = memo.entry(sub_bag.into()) {
            if *e.get() {
                found = true;
            }
        } else if contains_shiny_gold(sub_bag, bags, memo) {
            found = true;
        }

        if found {
            memo.insert(bag.into(), true);
            return true;
        }
    }

    memo.insert(bag.into(), false);
    return false;
}

fn part2(bags: &Bags) -> usize {
    let mut contains = HashMap::<String, usize>::new();
    let mut bags = bags.clone();

    while bags.len() > 0 {
        let empty = bags
            .iter()
            .filter(|(_, contents)| contents.len() == 0)
            .map(|(b, _)| String::from(b))
            .next()
            .unwrap();
        bags.remove(&empty);

        let empty_sub_bags = *contains.entry(empty.clone()).or_default();

        for (bag, contents) in &mut bags {
            if let Entry::Occupied(e) = contents.entry(empty.clone()) {
                let n = *e.get();
                e.remove();
                *contains.entry(bag.into()).or_insert(0) += n + n * empty_sub_bags;
            }
        }
    }

    return *contains.get("shiny gold").unwrap();
}
