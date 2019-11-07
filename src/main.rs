use algorithms::{Alg, Factorize};
use rug::Integer;
use std::time;
use structopt::StructOpt;

pub mod algorithms;

#[derive(StructOpt, Debug)]
#[structopt(name = "factorize")]
struct Options {
    /// Array of numbers to factorize
    #[structopt(required = true)]
    numbers: Vec<Integer>,

    /// Set to 'true' to assert correctness of factorizations
    #[structopt(long)]
    assert: bool,

    /// Algorithm to use for factorization
    #[structopt(long, default_value = "trial_division")]
    alg: Alg,
}

fn main() {
    let opts = Options::from_args();

    for number in opts.numbers {
        let timer = time::Instant::now();
        let mut factors = match opts.alg {
            Alg::TrialDivision => algorithms::TrialDivision::factor(number.clone()),
            Alg::BrentsRho => algorithms::BrentsRho::factor(number.clone()),
        };
        factors.sort();
        println!("{} => {:?}, took {:?}", number, factors, timer.elapsed());
        if opts.assert {
            assert_eq!(factors.iter().product::<Integer>(), number);
        }
    }
}
