use rug::{Assign, Integer};
use std::{fmt::Display, str::FromStr};

pub use brents_rho::BrentsRho;
pub use fermat::Fermat;
pub use trial_division::TrialDivision;

mod brents_rho;
mod fermat;
mod primality;
mod trial_division;

pub trait Factorize {
    fn factor(number: Integer) -> Vec<Integer>;
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

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "TRIALDIVISION" | "TRIAL DIVISION" | "TRIAL_DIVISION" => Ok(Alg::TrialDivision),
            "BRENTSRHO" | "BRENTS RHO" | "BRENTS_RHO" | "BRENTS'S RHO" => Ok(Alg::BrentsRho),
            "FERMAT" | "FERMATS" | "FERMAT'S" => Ok(Alg::Fermat),
            _ => std::result::Result::Err(ParseAlgError::UnknownAlg(s.into())),
        }
    }
}

fn gcd(mut a: Integer, mut b: Integer) -> Integer {
    let mut remainder_buffer = Integer::new();

    while b != 0 {
        remainder_buffer.assign(&a % &b);
        a.assign(&b);
        b.assign(&remainder_buffer);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use primality::*;

    const LOW_PRIMES: [u128; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    const LOW_COMPOSITES: [u128; 10] = [1, 4, 6, 8, 9, 10, 12, 14, 15, 16];
    const HIGH_PRIMES: [u128; 4] = [
        613_304_922_953,
        344_501_616_449,
        342_907_683_289,
        885_027_563_087,
    ];
    const HIGH_COMPOSITES: [u128; 4] = [
        885_027_563_083,
        1_440_456_103_525,
        1_439_896_263_162,
        2_287_220_812_551,
    ];

    #[test]
    fn miller_rabin_accurate_for_low_primes() {
        for prime in LOW_PRIMES.iter().map(|&n| Integer::from(n)) {
            assert_eq!(
                (&prime, miller_rabin(&prime.clone(), 40)),
                (&prime, MillerRabinResult::ProbablyPrime)
            );
        }
    }

    #[test]
    fn miller_rabin_accurate_for_low_composites() {
        for composite in LOW_COMPOSITES.iter().map(|&n| Integer::from(n)) {
            assert_eq!(
                (&composite, miller_rabin(&composite.clone(), 40)),
                (&composite, MillerRabinResult::Composite),
            );
        }
    }

    #[test]
    fn miller_rabin_accurate_for_high_primes() {
        for prime in HIGH_PRIMES.iter().map(|&n| Integer::from(n)) {
            assert_eq!(
                (&prime, miller_rabin(&prime.clone(), 40)),
                (&prime, MillerRabinResult::ProbablyPrime)
            );
        }
    }

    #[test]
    fn miller_rabin_accurate_for_high_composites() {
        for composite in HIGH_COMPOSITES.iter().map(|&n| Integer::from(n)) {
            assert_eq!(
                (&composite, miller_rabin(&composite.clone(), 40)),
                (&composite, MillerRabinResult::Composite)
            );
        }
    }

    #[test]
    fn trial_division_low_primes() {
        for prime in LOW_PRIMES.iter().map(|&n| Integer::from(n)) {
            assert_eq!(
                (&prime, TrialDivision::factor(prime.clone()).len()),
                (&prime, 1)
            );
        }
    }

    #[test]
    fn brents_rho_low_primes() {
        for prime in LOW_PRIMES.iter().map(|&n| Integer::from(n)) {
            assert_eq!(
                (&prime, BrentsRho::factor(prime.clone()).len()),
                (&prime, 1)
            );
        }
    }

    #[test]
    fn brents_rho_low_composites() {
        assert_eq!(BrentsRho::factor(Integer::from(12)), vec![2, 2, 3]);
        assert_eq!(BrentsRho::factor(Integer::from(15)), vec![3, 5]);
        assert_eq!(BrentsRho::factor(Integer::from(40)), vec![2, 2, 2, 5]);
        assert_eq!(BrentsRho::factor(Integer::from(42)), vec![2, 3, 7]);
    }

    #[test]
    fn fermat_low_primes() {
        for prime in LOW_PRIMES.iter().map(|&n| Integer::from(n)) {
            assert_eq!((&prime, Fermat::factor(prime.clone()).len()), (&prime, 1));
        }
    }

    #[test]
    fn fermat_low_composites() {
        assert_eq!(Fermat::factor(Integer::from(12)), vec![2, 2, 3]);
        assert_eq!(Fermat::factor(Integer::from(15)), vec![3, 5]);
        assert_eq!(Fermat::factor(Integer::from(40)), vec![2, 2, 2, 5]);
        assert_eq!(Fermat::factor(Integer::from(42)), vec![2, 3, 7]);
    }
}
