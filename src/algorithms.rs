use mod_exp::mod_exp;
use rand::Rng;

#[derive(PartialEq, Debug)]
enum MillerRabinResult {
    Composite,
    ProbablyPrime,
}

pub fn trial_division(mut n: u128) -> Vec<u128> {
    use std::iter;
    let mut factors: Vec<u128> = Vec::new();
    if n == 0 {
        return factors;
    }

    let candidates = iter::once(2).chain((3..).step_by(2));

    for candidate in candidates {
        if n <= 1 {
            break;
        }
        while n % candidate == 0 {
            factors.push(candidate);
            n /= candidate;
        }
    }

    factors
}

pub fn brents_rho(mut n: u128) -> Vec<u128> {
    let mut factors = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    while n != 1 {
        if let MillerRabinResult::ProbablyPrime = miller_rabin(n, 40) {
            factors.push(n);
            break;
        }

        // try Brent's rho until it succeeds
        for offset in 1.. {
            if let Ok(factor) = brents_rho_single(n, offset) {
                factors.push(factor);
                n /= factor;
                break;
            }
        }
    }
    factors.sort();
    factors
}

fn brents_rho_single(number: u128, offset: u32) -> Result<u128, ()> {
    let mut x_cycle = 2;
    let mut y_cycle = 2;
    let mut possible_factor = 1;

    let g = |x: u128, n: u128| ((x * x) + u128::from(offset)) % n;
    while possible_factor == 1 {
        x_cycle = g(x_cycle, number);
        y_cycle = g(g(y_cycle, number), number);
        possible_factor = gcd((x_cycle as i128 - y_cycle as i128).abs() as u128, number);
    }
    if possible_factor == number {
        Err(())
    } else {
        Ok(possible_factor)
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn miller_rabin(number: u128, iterations: u32) -> MillerRabinResult {
    if number % 2 == 0 || number <= 3 {
        return MillerRabinResult::Composite;
    }

    let mut rng = rand::thread_rng();
    let (exponent, scalar) = factor_out_twos(number - 1);
    'witness: for _ in 0..iterations {
        let random_witness: u128 = rng.gen_range(2, number - 1);
        let mut x = mod_exp(random_witness, scalar, number);
        if x == 1 || x == number - 1 {
            continue 'witness;
        } else {
            for _ in 0..exponent - 1 {
                x = mod_exp(x, 2, number);
                if x == number - 1 {
                    continue 'witness;
                }
            }
            return MillerRabinResult::Composite;
        }
    }

    MillerRabinResult::ProbablyPrime
}

/// Represent a value `n` as `2^s * d`
///
/// Returns a tuple containing `(s, d)`
fn factor_out_twos(mut n: u128) -> (u128, u128) {
    let mut s = 0;
    while n % 2 == 0 {
        s += 1;
        n /= 2;
    }

    (s, n)
}

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
        assert_eq!(number, factors.iter().product());
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
        let factors = trial_division(number);
        assert_eq!(number, factors.iter().product());
    }
}
