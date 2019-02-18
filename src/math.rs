use rand::*;

pub fn trial_division(n: &u128) -> Vec<u128> {
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

pub fn miller_rabin(n: &u128) -> bool {
    // simple fast checks
    if *n < 6_u128 {
        [false, false, true, true, false, true][*n as usize]
    } else if n & 1 == 0 {
        false
    } else {
        let mut s: u128 = 0;
        let mut d: u128 = n - 1;

        while d & 1 == 0 {
            s = s + 1;
            d = d >> 1;
        }

        let mut rng = rand::thread_rng();

        for i in 0..7

        false
    }

    /// // Type annotation requires two types, the type and distribution; the
    /// // distribution can be inferred.
    /// let y = rng.sample::<u16, _>(Uniform::new(10, 15));
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

pub fn gcd(a: u128, b: u128) -> u128 {
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
