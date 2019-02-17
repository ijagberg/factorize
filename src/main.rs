use rand::*;
use std::env;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut numbers: Vec<u128> = Vec::new();
    for arg in &args[1..] {
        match arg.parse::<u128>() {
            Ok(n) => numbers.push(n),
            Err(e) => {
                eprintln!("An error occurred while parsing: {:?}", e);
                return;
            }
        };
    }

    for number in numbers {
        let timer = time::Instant::now();
        let factors = trial_division(&number);
        let mut product: u128 = 1;
        for factor in &factors {
            product *= factor;
        }
        println!("{} => {:?}, took {:?}", number, factors, timer.elapsed());
        assert_eq!(number, product);
    }
}

fn trial_division(n: &u128) -> Vec<u128> {
    let mut factors: Vec<u128> = Vec::new();
    let mut n_copy = n.clone();
    // special case for 2
    while n_copy % 2 == 0 {
        factors.push(2);
        n_copy /= 2;
    }

    let mut d = 3;
    while d as f64 <= (n_copy as f64).sqrt() + 2_f64 {
        if n_copy % d == 0 {
            factors.push(d);
            n_copy /= d;
        } else {
            d += 2;
        }
    }
    if n_copy > 1 {
        factors.push(n_copy);
    }
    factors
}

fn miller_rabin(n: &u128) -> bool {
    // simple fast checks
    if [2_u128, 3_u128, 5_u128, 7_u128].contains(&n) || n % 2 == 0 {
        true
    } else {
        let mut d: u128 = n - 1;
        let mut r: u128 = 1;
        let n_minus_one: u128 = n - 1;

        while d % 2 != 0 {
            r += 1;
            d /= 2;
        }

        let upper_limit: u128 = n - 3;
        let power: u128 = 7;

        for i in 1..10 {
            let random = rand::thread_rng().next_u64();
            // TODO: modular_exponent()
        }

        false
    }

    //     mpz_t d, r, modulus, nMinusOne;

    //     mpz_t a, x, upperLim;
    //     mpz_init(a);
    //     mpz_init(x);

    //     mpz_init(upperLim);
    //     mpz_sub_ui(upperLim, n, 3); // For upper limit, range [2, n-2] = [0, n-3] + 2

    //     mpz_t power;
    //     mpz_init_set_ui(power, 7);

    //     for (int i = 0; i < 10; i++) {
    //         // Randomize number in range [2, n-2] //
    //         modularExp(a, a, power, upperLim); // random ^ 7 % n
    //         mpz_add_ui(a, a, 2); // n += 2
    //         //###//

    //         // a ^ d % n //
    //         modularExp(x, a, d, n);
    //         if (mpz_cmp_ui(x, 1) == 0 || mpz_cmp(x, nMinusOne) == 0) {
    //             // If x == 1 || x == n-1
    //             continue;
    //         }
    //         //###//

    //         mpz_t j; // Counter for loop
    //         mpz_init_set_ui(j, 1);

    //         while (mpz_cmp(j, r) < 0) { // While j < r
    //             // x ^ 2 % n
    //             mpz_t tmp_two;
    //             mpz_init_set_ui(tmp_two, 2);
    //             modularExp(x, x, tmp_two, n);
    //             //###//

    //             if (mpz_cmp_ui(x, 1) == 0) return false;
    //             if (mpz_cmp(x, nMinusOne) == 0) {
    //                 break;
    //             }

    //             mpz_add_ui(j, j, 1);
    //         }
    //         if (mpz_cmp(j, r) == 0) return false;
    //     }

    //     return true;
    // }
}

fn gcd(a: u128, b: u128) -> u128 {
    let mut a_mut = a;
    let mut b_mut = b;
    let mut remainder: u128;
    while b_mut != 0 {
        remainder = a_mut % b_mut;
        a_mut = b_mut;
        b_mut = remainder;
    }
    return a_mut;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(20, 10), 10);
        assert_eq!(gcd(25, 10), 5);
        assert_eq!(gcd(9, 13), 1);
    }
}
