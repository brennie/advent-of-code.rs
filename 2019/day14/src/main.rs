use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let rxns = read_input()?;

    println!("part 1: {}", ore_per_n_fuel(1, &rxns));

    {
        let mut lower_bound = 0;
        let mut upper_bound = 1000000000000i64;

        while lower_bound < upper_bound {
            let midpoint = (upper_bound + lower_bound + 1 ) / 2;

            let cost = ore_per_n_fuel(midpoint, &rxns);

            if cost < 1000000000000 {
                lower_bound = midpoint;
            } else {
                upper_bound = midpoint - 1;
            }

        }

        println!("part 2: {}", lower_bound);
    }

    Ok(())
}

fn ore_per_n_fuel(fuel: i64, rxns: &HashMap<Compound, Vec<Compound>>) -> i64 {
    let mut ore_required = 0;
    let mut compounds = HashMap::<String, i64>::new();

    // negative represents surplus.
    compounds.insert("FUEL".into(), fuel);

    loop {
        if let Some((compound, defecit)) = compounds
            .iter()
            .filter_map(|(c, n)| {
                if *n > 0 {
                    Some((String::from(c), *n))
                } else {
                    None
                }
            })
            .next()
        {
            compounds.remove(&compound);

            let (result, reqs) = rxns.iter().find(|(Compound(_, m), _)| *m == compound).unwrap();

            let mut mult = defecit / result.0;
            if defecit % result.0 > 0 {
                mult += 1;
            }

            for req in reqs {
                if req.1 == "ORE" {
                    ore_required += mult * req.0;
                } else {
                    *compounds.entry(req.1.clone()).or_default() += mult * req.0;
                }
            }

            let total = result.0 * mult;
            if total != defecit {
                compounds.insert(compound, defecit - total);
            }
        } else {
            break;
        }
    }

    ore_required
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Compound(i64, String);

fn read_input() -> Result<HashMap<Compound, Vec<Compound>>, Box<dyn Error>> {
    let reader = BufReader::new(File::open("input")?);
    let mut rxns = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts = line.split(" => ").collect::<Vec<_>>();
        let left = parts[0];
        let right = parts[1];

        let mut left_compounds = vec![];

        for c in left.split(", ") {
            let idx = c.find(" ").unwrap();

            let n: i64 = str::parse(&c[..idx])?;
            let s = &c[idx + 1..];

            left_compounds.push(Compound(n, s.into()));
        }

        let result = {
            let idx = right.find(" ").unwrap();

            let n: i64 = str::parse(&right[..idx])?;
            let s = &right[idx + 1..];

            Compound(n, s.into())
        };

        rxns.insert(result, left_compounds);
    }

    Ok(rxns)
}
