use super::primality;
use crate::Factorize;
use rug::{Assign, Integer};
use std::collections::VecDeque;

pub struct Fermat;

impl Fermat {
    fn ceiling_root(number: &Integer) -> Integer {
        let root = number.clone().root(2);
        let squared = Integer::from(&root * &root);
        if &squared == number {
            Integer::from(root + 1)
        } else {
            root
        }
    }
}

impl Factorize for Fermat {
    fn factor(mut number: Integer) -> Vec<Integer> {
        let mut factors = Vec::new();
        let mut to_factorize = VecDeque::new();

        // Remove all twos first
        while number.is_divisible_u(2) {
            factors.push(Integer::from(2));
            number /= 2;
        }
        to_factorize.push_back(number);

        'factorize: while let Some(number) = to_factorize.pop_front() {
            if number == 1 {
                continue;
            }
            if let primality::MillerRabinResult::ProbablyPrime =
                primality::miller_rabin(&number, 40)
            {
                factors.push(number);
                continue;
            }
            let mut a = Fermat::ceiling_root(&number);
            let mut b = Integer::new();
            loop {
                b.assign(&a * &a - &number);
                if b.is_perfect_square() {
                    let factor = Integer::from(&a - b.clone().root(2));
                    let twin = Integer::from(&number / &factor);
                    to_factorize.push_back(factor);
                    to_factorize.push_back(twin);
                    continue 'factorize;
                } else {
                    a += 1;
                }
            }
        }

        factors
    }
}
