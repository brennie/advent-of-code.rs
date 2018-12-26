use failure::Error;

use day12::read_input;

fn main() -> Result<(), Error> {
    let mut garden = read_input()?;

    for _ in 0..20 {
        garden.next();
    }

    println!("{}", garden.score());

    Ok(())
}
