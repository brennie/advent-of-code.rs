use std::process::exit;

use day05::{react_polymer, read_input};

fn main() {
    match read_input().map(|polymer| react_polymer(&polymer)) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(result) => println!("{}", result.len()),
    }
}
