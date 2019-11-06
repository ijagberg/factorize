use mod_exp::mod_exp;
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
        if remainder_buffer == 0 {
            break;
        }
        n /= 2;
    }

    let mut to_factorize = VecDeque::new();
    to_factorize.push_back(n);
    'factorize: while let Some(number) = to_factorize.pop_front() {
        if n == 1 {
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
                to_factorize.push_back(factor);
                to_factorize.push_back(number / factor);
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
        abs_diff_buffer.assign(abs_diff_buffer.abs());
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
        remainder_buffer.assign(a % b);;
        a.assign(b);
        b.assign(remainder_buffer);
    }
    a
}

fn miller_rabin(number: &Integer, iterations: u32) -> MillerRabinResult {
    if number == &2 || number == &3 {
        return MillerRabinResult::ProbablyPrime;
    }
    let mut remainder_buffer = Integer::new();
    remainder_buffer.assign(number % 2);
    if remainder_buffer == 0 || number <= &3 {
        return MillerRabinResult::Composite;
    }

    let one = Integer::from(1);
    let two = Integer::from(2);
    let mut number_minus_one_buffer = Integer::new();
    number_minus_one_buffer.assign(number - 1);
    let mut rand = RandState::new();
    let mut base_buffer = Integer::new();
    let (exponent, scalar) = factor_out_twos(number_minus_one_buffer);
    'witness: for _ in 0..iterations {
        let mut random_witness: Integer = number_minus_one_buffer.random_below(&mut rand);
        random_witness.assign(random_witness.pow_mod(&two, &number).unwrap()); 

        if random_witness == 1 || random_witness == number_minus_one_buffer {
            continue 'witness;
        } else {
            let mut loop_count = Integer::from(0);
            let mut exp_minus_one_buffer = Integer::new();
            exp_minus_one_buffer.assign(exponent - one);
            loop {
                if loop_count < exp_minus_one_buffer {
                    return MillerRabinResult::Composite;
                }

                random_witness.assign(random_witness.pow_mod(&two, &number).unwrap());
                if random_witness == number_minus_one_buffer {
                    continue 'witness;
                }
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
        remainder_buffer.assign(n % 2);
        if remainder_buffer == 0 {
            break;
        }
    }

    while n % 2 == 0 {
        s += 1;
        n /= 2;
    }

    (s, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOW_PRIMES: [u128; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];

    #[test]
    fn test_pollards_rho() {
        let numbers: Vec<u128> = vec![
            1,
            2,
            3,
            10,
            100,
            150,
            50000,
            1_234_567_890,
            948_347_928_371_874,
        ];
        for number in numbers {
            let factors = brents_rho(number);
            assert_eq!(number, factors.iter().product::<u128>());
        }
    }

    #[test]
    fn test_miller_rabin() {
        assert_eq!(miller_rabin(13, 40), MillerRabinResult::ProbablyPrime);
        assert_eq!(miller_rabin(221, 40), MillerRabinResult::Composite);
    }

    #[test]
    fn test_factor_out_twos_1() {
        assert_eq!(factor_out_twos(221 - 1), (2, 55));
        assert_eq!(factor_out_twos(13 - 1), (2, 3));
    }

    #[test]
    fn trial_div_should_factorize_correctly() {
        let numbers: Vec<Integer> = vec![
            1_u128,
            2_u128,
            3_u128,
            10_u128,
            100_u128,
            150_u128,
            50000_u128,
            1_234_567_890_u128,
            948_347_928_371_874_u128,
        ]
        .iter()
        .map(|&n| Integer::from(n))
        .collect();
        for number in numbers {
            let factors = trial_division(number.clone());
            assert_eq!(number, factors.iter().product::<Integer>());
        }
    }

    #[test]
    fn trial_division_low_primes() {
        for number in LOW_PRIMES.iter().map(|&low_prime| Integer::from(low_prime)) {
            let factors = trial_division(number.clone());
            assert_eq!(factors.len(), 1);
        }
    }

    #[test]
    fn brents_rho_low_primes() {
        for number in &LOW_PRIMES {
            let factors = brents_rho(*number);
            assert_eq!(factors.len(), 1);
        }
    }
}
