use std::time;
use structopt::StructOpt;
pub mod algorithms;

#[derive(StructOpt, Debug)]
#[structopt(name = "factorize")]
struct Options {
    #[structopt(required = true)]
    numbers: Vec<u128>,

    #[structopt(long)]
    assert: bool
}

fn main() {
    let opts = Options::from_args();

    for number in opts.numbers {
        let timer = time::Instant::now();
        let factors = algorithms::trial_division(number);
        println!("{} => {:?}, took {:?}", number, factors, timer.elapsed());
        if opts.assert {
            assert_eq!(number, factors.iter().product());
        }
    }
}
