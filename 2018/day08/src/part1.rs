use std::process::exit;

use day08::{read_tree, Node};

fn main() {
    match read_tree().map(sum_meta) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1)
        }

        Ok(sum) => println!("{}", sum),
    }
}

fn sum_meta(node: Node) -> u32 {
    let mut stack = vec![node];
    let mut sum = 0;

    while let Some(n) = stack.pop() {
        for meta in n.meta {
            sum += meta;
        }

        for child in n.children {
            stack.push(child);
        }
    }

    sum
}
