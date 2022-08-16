use factorize::{Factorize, TrialDivision, Fermat, BrentsRho};
use rug::Integer;
use std::fmt::Display;
use std::str::FromStr;
use std::time;
use structopt::StructOpt;

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

#[derive(Debug)]
pub enum Alg {
    TrialDivision,
    BrentsRho,
    Fermat,
}

impl Display for Alg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Alg::TrialDivision => "trial division",
                Alg::BrentsRho => "Brent's Rho",
                Alg::Fermat => "Fermat",
            }
        )
    }
}

#[derive(Debug)]
pub enum ParseAlgError {
    UnknownAlg(String),
}

impl std::error::Error for ParseAlgError {}

impl std::fmt::Display for ParseAlgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ParseAlgError::UnknownAlg(alg) => write!(f, "unknown algorithm '{}'", alg),
        }
    }
}

impl FromStr for Alg {
    type Err = ParseAlgError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "TRIALDIVISION" | "TRIAL DIVISION" | "TRIAL_DIVISION" => Ok(Alg::TrialDivision),
            "BRENTSRHO" | "BRENTS RHO" | "BRENTS_RHO" | "BRENTS'S RHO" => Ok(Alg::BrentsRho),
            "FERMAT" | "FERMATS" | "FERMAT'S" => Ok(Alg::Fermat),
            _ => Result::Err(ParseAlgError::UnknownAlg(s.into())),
        }
    }
}

fn main() {
    let opts = Options::from_args();

    for number in opts.numbers {
        let timer = time::Instant::now();
        let arg_number = number.clone();
        let mut factors = match opts.alg {
            Alg::TrialDivision => TrialDivision::factor(arg_number),
            Alg::BrentsRho => BrentsRho::factor(arg_number),
            Alg::Fermat => Fermat::factor(arg_number),
        };
        factors.sort();
        println!(
            r#"{} => {:?}, took {:?} with "{}""#,
            number,
            factors,
            timer.elapsed(),
            opts.alg
        );
        if opts.assert {
            assert_eq!(factors.iter().product::<Integer>(), number);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprime() {
        for n in 1..1000 {
            let facts = algorithms::TrialDivision::factor(Integer::from(n));
            if let algorithms::primality::MillerRabinResult::ProbablyPrime =
                algorithms::primality::miller_rabin(&Integer::from(facts.len()), 40)
            {
                // println!("{} has {} factors", n, facts.len());
            } else if facts.len() > Integer::from(1) {
                println!("{} has {} factors", n, facts.len());
            }
        }
    }
}
