use std::str::FromStr;
use std::time;
use structopt::StructOpt;
pub mod algorithms;

#[derive(StructOpt, Debug)]
#[structopt(name = "factorize")]
struct Options {
    /// Array of numbers to factorize
    #[structopt(required = true)]
    numbers: Vec<u128>,

    /// Set to 'true' to assert correctness of factorizations
    #[structopt(long)]
    assert: bool,

    /// Algorithm to use for factorization
    #[structopt(long, default_value = "trial_division")]
    alg: Alg,
}

#[derive(Debug)]
enum Alg {
    TrialDivision,
    BrentsRho,
}

#[derive(Debug)]
enum ParseError {
    UnknownAlg(String),
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ParseError::UnknownAlg(alg) => write!(f, "unknown algorithm '{}'", alg),
        }
    }
}

impl FromStr for Alg {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "TRIALDIVISION" | "TRIAL DIVISION" | "TRIAL_DIVISION" => Ok(Alg::TrialDivision),
            "BRENTSRHO" | "BRENTS RHO" | "BRENTS_RHO" | "BRENTS'S RHO" => Ok(Alg::BrentsRho),
            _ => std::result::Result::Err(ParseError::UnknownAlg(s.into())),
        }
    }
}

fn main() {
    let opts = Options::from_args();

    for number in opts.numbers {
        let timer = time::Instant::now();
        let factors = match opts.alg {
            Alg::TrialDivision => algorithms::trial_division(number),
            Alg::BrentsRho => algorithms::brents_rho(number),
        };
        println!("{} => {:?}, took {:?}", number, factors, timer.elapsed());
        if opts.assert {
            assert_eq!(number, factors.iter().product());
        }
    }
}
