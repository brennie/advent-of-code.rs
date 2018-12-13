use structopt::StructOpt;

use day09::{run_game, Options};

fn main() {
    let opts = Options::from_args();
    println!("{}", run_game(opts));
}
