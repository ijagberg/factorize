use rug::{rand::RandState, Assign, Integer};

#[derive(PartialEq, Debug)]
pub enum MillerRabinResult {
    /// Definitely a composite number
    Composite,
    /// Probably a prime number
    ProbablyPrime,
}

pub fn miller_rabin(number: &Integer, iterations: u32) -> MillerRabinResult {
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
