#[allow(unused_imports)]
use rug::{rand::RandState, Assign, Integer};
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum MillerRabinResult {
    /// Definitely a composite number
    Composite,
    /// Probably a prime number
    ProbablyPrime,
}

pub fn trial_division(mut n: Integer) -> Vec<Integer> {
    use std::iter;
    let mut factors = Vec::new();
    let one = Integer::from(1);
    if n == 0 {
        return factors;
    }

    let candidates = iter::once(2).chain((3..).step_by(2));

    let mut remainder_buffer = Integer::new();
    for candidate in candidates {
        if &n <= &one {
            break;
        } else {
            loop {
                remainder_buffer.assign(&n % candidate);
                if remainder_buffer != 0 {
                    break;
                }
                factors.push(Integer::from(candidate));
                n /= candidate;
            }
        }
    }

    factors
}

pub fn brents_rho(mut n: Integer) -> Vec<Integer> {
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
        if let MillerRabinResult::ProbablyPrime = miller_rabin(&number, 40) {
            factors.push(number);
            continue;
        }

        // try Brent's rho until it succeeds (panic if this takes more than 100 iterations)
        for offset in 1..100 {
            if let Ok(factor) = brents_rho_single(&number, offset) {
                // Brent's rho returned a factor of n, but it might not be prime
                // so add it and its twin to the deque
                let twin = Integer::from(&number / &factor);
                to_factorize.push_back(factor);
                to_factorize.push_back(twin);
                continue 'factorize;
            } else {
                eprintln!(
                    "Brent's rho algorithm failed for number={}, attempting again with offset={}",
                    number,
                    offset + 1
                );
            }
        }
        panic!("Brent's rho took more than 100 iterations");
    }
    factors.sort();
    factors
}

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

fn gcd(mut a: Integer, mut b: Integer) -> Integer {
    let mut remainder_buffer = Integer::new();

    while b != 0 {
        remainder_buffer.assign(&a % &b);
        a.assign(&b);
        b.assign(&remainder_buffer);
    }
    a
}

fn miller_rabin(number: &Integer, iterations: u32) -> MillerRabinResult {
    if number == &2 || number == &3 {
        return MillerRabinResult::ProbablyPrime;
    }

    let remainder_buffer = Integer::from(number % Integer::from(2));
    if remainder_buffer == 0 || number < &3 {
        return MillerRabinResult::Composite;
    }

    let two = Integer::from(2);
    let number_minus_one = Integer::from(number - 1);
    let number_minus_three = Integer::from(&number_minus_one - &two);
    let mut rand = RandState::new();
    let (exponent, scalar) = factor_out_twos(number_minus_one.clone());
    'witness: for _ in 0..iterations {
        let mut random_witness =
            Integer::from(&Integer::from(number_minus_three.random_below_ref(&mut rand)) + &two);
        random_witness.assign(random_witness.clone().pow_mod(&scalar, &number).unwrap());

        if random_witness == 1 || random_witness == number_minus_one {
            continue 'witness;
        } else {
            let mut loop_count = Integer::from(0);
            let exp_minus_one = Integer::from(&exponent - 1);
            loop {
                if loop_count >= exp_minus_one {
                    return MillerRabinResult::Composite;
                }

                random_witness.assign(random_witness.clone().pow_mod(&two, &number).unwrap());
                if random_witness == number_minus_one {
                    continue 'witness;
                }
                loop_count += 1;
            }
        }
    }

    MillerRabinResult::ProbablyPrime
}

/// Represent a value `n` as `2^s * d`
///
/// Returns a tuple containing `(s, d)`
fn factor_out_twos(mut n: Integer) -> (Integer, Integer) {
    let mut s = Integer::from(0);
    let mut remainder_buffer = Integer::new();

    loop {
        remainder_buffer.assign(&n % 2);
        if remainder_buffer == 0 {
            break;
        }
    }

    loop {
        remainder_buffer.assign(&n % 2);
        if remainder_buffer != 0 {
            break;
        }
        s += 1;
        n /= 2;
    }

    (s, n)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert_eq!((&prime, trial_division(prime.clone()).len()), (&prime, 1));
        }
    }

    #[test]
    fn brents_rho_factors_low_primes_correctly() {
        for prime in LOW_PRIMES.iter().map(|&n| Integer::from(n)) {
            assert_eq!((&prime, brents_rho(prime.clone()).len()), (&prime, 1));
        }
    }

    #[test]
    fn brents_rho_low_composites() {
        assert_eq!(brents_rho(Integer::from(12)), vec![2, 2, 3]);
        assert_eq!(brents_rho(Integer::from(15)), vec![3, 5]);
        assert_eq!(brents_rho(Integer::from(40)), vec![2, 2, 2, 5]);
        assert_eq!(brents_rho(Integer::from(42)), vec![2, 3, 7]);
    }
}
