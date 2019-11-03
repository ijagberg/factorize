use std::time;
use structopt::StructOpt;
pub mod algorithms;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Options {
    numbers: Vec<u128>,
}

fn main() {
    let opts = Options::from_args();

    for number in opts.numbers {
        let timer = time::Instant::now();
        let factors = algorithms::trial_division(number);
        println!("{} => {:?}, took {:?}", number, factors, timer.elapsed());
    }
}
