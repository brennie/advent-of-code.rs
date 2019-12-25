use std::mem::swap;

use failure::Error;

use day12::{read_input, Garden};

fn main() -> Result<(), Error> {
    let mut garden = read_input()?;

    // By experiment, the automata becomes linear eventually.
    let mut last_trimmed_pots = trim_pots(&garden).collect::<Vec<_>>();
    let mut trimmed_pots = Vec::with_capacity(last_trimmed_pots.len());
    garden.next();

    let mut i = 1u64;
    loop {
        trimmed_pots.extend(trim_pots(&garden));

        if trimmed_pots == last_trimmed_pots {
            break;
        }

        swap(&mut last_trimmed_pots, &mut trimmed_pots);
        trimmed_pots.clear();

        garden.next();
        i += 1;
    }

    let last_score = garden.score() as u64;
    garden.next();
    let next_score = garden.score() as u64;

    let score = last_score + (50000000000u64 - i) * (next_score - last_score);

    println!("{}", score);

    Ok(())
}

fn trim_pots(g: &Garden) -> impl Iterator<Item = bool> + '_ {
    g.pots().map(|(_, pot)| pot).skip_while(|pot| !pot)
}
