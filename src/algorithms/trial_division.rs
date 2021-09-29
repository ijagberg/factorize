use crate::algorithms::Factorize;
use rug::{Assign, Integer};

pub struct TrialDivision;

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
