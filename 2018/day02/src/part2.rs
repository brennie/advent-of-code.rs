use std::process::exit;

use day02_2018::{read_ids, Error, Result};

fn run() -> Result<String> {
    let ids = read_ids()?;

    for i in 0..ids.len() - 1 {
        let str_i = &ids[i];

        'inner: for j in i + 1..ids.len() {
            let str_j = &ids[j];

            if ids[i].len() != ids[j].len() {
                return Err(Error::LengthMisatch((*str_i).clone(), (*str_j).clone()));
            }

            let mut difference = None;

            for (pos, (a, b)) in str_i.chars().zip(str_j.chars()).enumerate() {
                if a != b {
                    if difference.is_some() {
                        continue 'inner;
                    }

                    difference = Some(pos);
                }
            }

            if let Some(pos) = difference {
                let target_id = ids[i]
                    .chars()
                    .enumerate()
                    .filter(|(k, _)| *k != pos)
                    .map(|(_, c)| c)
                    .collect();

                return Ok(target_id);
            }
        }
    }

    Err(Error::NoMatch)
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(result) => println!("{}", result),
    }
}
