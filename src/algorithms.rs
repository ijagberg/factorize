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

pub fn pollards_rho(mut n: u128) -> Vec<u128> {
    let mut factors = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let g = |x: u128, n: u128| ((x * x) + 1) % n;
    while n != 1 {
        let mut x = 2;
        let mut y = 2;
        let mut d = 1;

        while d == 1 {
            x = g(x, n);
            y = g(g(y, n), n);
            d = gcd((x as i128 - y as i128).abs() as u128, n);
            if d == n {
                panic!("pollards rho failed for n = {}", n);
            } else {
                dbg!(&d);
                factors.push(d);
                n /= d;
            }
        }
    }
    factors
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    let mut remainder = 0;
    while b != 0 {
        remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn miller_rabin(n: u128, k: u32) -> MillerRabinResult {
    if n % 2 == 0 || n <= 3 {
        return MillerRabinResult::Composite;
    }

    let mut rng = rand::thread_rng();
    let (r, d) = factor_out_twos(n - 1);
    dbg!((r, d));
    'witness: for _ in 0..k {
        let random_witness: u128 = rng.gen_range(2, n - 1);
        //let random_witness = 174;
        dbg!(&random_witness);
        let mut x = mod_exp(random_witness, d, n);
        dbg!(&x);
        if x == 1 || x == n - 1 {
            continue 'witness;
        } else {
            for _ in 0..r - 1 {
                x = mod_exp(x, 2, n);
                dbg!(&x);
                if x == n - 1 {
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
    let mut factors = pollards_rho(20);
    factors.sort();
    assert_eq!(factors, vec![2, 2, 5]);
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
