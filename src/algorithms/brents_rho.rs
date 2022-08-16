use super::primality;
use crate::{gcd, Factorize};
use rug::{Assign, Integer};
use std::collections::VecDeque;

pub struct BrentsRho;

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

        while n.is_divisible_u(2) {
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
        factors
    }
}
