use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const YOU: &str = "YOU";
const SAN: &str = "SAN";

pub type Dag = HashMap<String, String>;

fn read_input() -> Result<Dag, Box<dyn Error>> {
    let mut result = HashMap::new();

    let reader = BufReader::new(File::open("input")?);
    for line in reader.lines() {
        let line = line?;

        if line.len() == 0 {
            continue;
        }

        let parts = line.split(")").collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);
        result.insert(parts[1].into(), parts[0].into());
    }

    Ok(result)
}

fn walk_tree(tree: &Dag) -> usize {
    let mut count = 0;

    for orbiter in tree.keys() {
        let mut orbitee = tree.get(orbiter);

        while let Some(next) = orbitee {
            count += 1;
            orbitee = tree.get(next);
        }
    }

    count
}

fn parents(tree: &Dag, node: &str) -> HashSet<String> {
    let mut parents = HashSet::new();

    let mut node = node;
    while let Some(next_node) = tree.get(node) {
        parents.insert(next_node.into());
        node = next_node;
    }

    parents
}

fn find_min_path(tree: &Dag) -> Option<usize> {
    let san_parents = parents(&tree, SAN);
    let you_parents = parents(&tree, YOU);

    let intersections = san_parents.intersection(&you_parents);

    let mut min_path = None;

    for intersection in intersections {
        let you_len = find_path(tree, YOU, intersection).unwrap();
        let san_len = find_path(tree, SAN, intersection).unwrap();

        let len = you_len + san_len;
        if let Some(old_len) = min_path {
            min_path = Some(std::cmp::min(old_len, len));
        } else {
            min_path = Some(len);
        }
    }

    min_path
}

fn find_path(tree: &Dag, start: &str, dest: &str) -> Option<usize> {
    let mut len = 0;

    let mut node = start;
    while let Some(next) = tree.get(node) {
        if next == dest {
            return Some(len);
        }

        len += 1;
        node = next;
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let orbits = read_input()?;

    println!("part 1: {}", walk_tree(&orbits));
    println!("part 2: {:?}", find_min_path(&orbits));

    Ok(())
}
