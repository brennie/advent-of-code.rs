use std::process::exit;

use day08::{read_tree, Node};

fn main() {
    match read_tree().map(|n| sum_meta(&n)) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1)
        }

        Ok(sum) => println!("{}", sum),
    }
}

fn sum_meta(node: &Node) -> u32 {
    if node.children.len() == 0 { 
        node.meta.iter().sum::<u32>()
    } else {
        let mut sum = 0;

        for meta in &node.meta {
            let meta = *meta as usize;
            if meta == 0 || meta > node.children.len() {
                continue
            }

            sum += sum_meta(&node.children[meta - 1]);
        }

        sum
    }
}
