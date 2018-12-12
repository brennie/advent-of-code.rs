use std::collections::HashSet;
use std::process::exit;

use day05::{react_polymer, read_input, Unit};

fn main() {
    match read_input().map(|polymer| minimize_polymer(&polymer)) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(result) => println!("{}", result),
    }
}

fn minimize_polymer(polymer: &[Unit]) -> usize {
    let units_seen = polymer
        .iter()
        .map(|unit| unit.value)
        .collect::<HashSet<_>>();

    units_seen
        .into_iter()
        .map(|value| {
            polymer
                .iter()
                .filter(|unit| unit.value != value)
                .map(|&unit| unit)
                .collect::<Vec<_>>()
        })
        .map(|polymer| react_polymer(&polymer).len())
        .min()
        .expect("No polymer present?")
}
