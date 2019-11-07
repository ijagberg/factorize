use rug::{Assign, Integer};
use std::collections::VecDeque;
use std::str::FromStr;

mod primality;

pub trait Factorize {
    fn factor(number: Integer) -> Vec<Integer>;
}

#[derive(Debug)]
pub enum Alg {
    TrialDivision,
    BrentsRho,
}

pub struct TrialDivision {}

impl Factorize for TrialDivision {
    fn factor(mut number: Integer) -> Vec<Integer> {
        use std::iter;
        let mut factors = Vec::new();
        if number == 0 {
            return factors;
        }

        let candidates = iter::once(2).chain((3..).step_by(2));

        let mut remainder_buffer = Integer::new();
        for candidate in candidates {
            if number <= 1 {
                break;
            } else {
                loop {
                    remainder_buffer.assign(&number % candidate);
                    if remainder_buffer != 0 {
                        break;
                    }
                    factors.push(Integer::from(candidate));
                    number /= candidate;
                }
            }
        }

        factors
    }
}

pub struct BrentsRho {}

impl BrentsRho {
    fn brents_rho_single(number: &Integer, offset: u16) -> Result<Integer, ()> {
        let mut x_cycle = Integer::from(2);
        let mut y_cycle = Integer::from(2);
        let mut possible_factor = Integer::from(1);

        let g = |x: &Integer, n: &Integer| {
            let mut multiplication_buffer = Integer::new();
            multiplication_buffer.assign(x * x);
            multiplication_buffer += offset;
            multiplication_buffer % n
        };

        let mut abs_diff_buffer = Integer::new();
        while possible_factor == 1 {
            x_cycle.assign(g(&x_cycle, number));
            y_cycle.assign(g(&g(&y_cycle, number), number));

            abs_diff_buffer.assign(&x_cycle - &y_cycle);
            abs_diff_buffer.assign(abs_diff_buffer.clone().abs());
            possible_factor.assign(gcd(abs_diff_buffer.clone(), number.clone()));
        }
        if &possible_factor == number {
            Err(())
        } else {
            Ok(possible_factor)
        }
    }
}

impl Factorize for BrentsRho {
    fn factor(mut n: Integer) -> Vec<Integer> {
        let mut factors = Vec::new();

        let mut remainder_buffer = Integer::new();
        loop {
            remainder_buffer.assign(&n % 2);
            if remainder_buffer != 0 {
                break;
            }
            factors.push(Integer::from(2));
            n /= 2;
        }

        let mut to_factorize = VecDeque::new();
        to_factorize.push_back(n);
        'factorize: while let Some(number) = to_factorize.pop_front() {
            if number == 1 {
                return factors;
            }
            if let primality::MillerRabinResult::ProbablyPrime =
                primality::miller_rabin(&number, 40)
            {
                factors.push(number);
                continue;
            }

            // try Brent's rho until it succeeds (panic if this takes more than 100 iterations)
            for offset in 1..100 {
                if let Ok(factor) = Self::brents_rho_single(&number, offset) {
                    // Brent's rho returned a factor of n, but it might not be prime
                    // so add it and its twin to the deque
                    let twin = Integer::from(&number / &factor);
                    to_factorize.push_back(factor);
                    to_factorize.push_back(twin);
                    continue 'factorize;
                }
            }
            panic!("Brent's rho took more than 100 iterations for {}", &number);
        }
        factors.sort();
        factors
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
                (&composite, MillerRabinResult::Composite)
            );
        }
    }

    #[test]
    fn trial_division_factors_low_primes_correctly() {
        for prime in LOW_PRIMES.iter().map(|&n| Integer::from(n)) {
            assert_eq!(
                (&prime, TrialDivision::factor(prime.clone()).len()),
                (&prime, 1)
            );
        }
    }

    #[test]
    fn brents_rho_factors_low_primes_correctly() {
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
}
