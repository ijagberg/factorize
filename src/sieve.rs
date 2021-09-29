pub struct Sieve {
    nums: Vec<bool>,
}

impl Sieve {
    pub fn new(len: usize) -> Sieve {
        let mut nums = vec![true; len + 1];
        nums[0] = false;
        nums[1] = false;
        for b in 2..=len {
            for multiple_of_b in (b..=len).step_by(b).skip(1) {
                nums[multiple_of_b] = false;
            }
        }

        Self { nums }
    }

    pub fn primes(&self) -> impl Iterator<Item = usize> + '_ {
        self.nums
            .iter()
            .enumerate()
            .filter_map(|(idx, &b)| if b { Some(idx) } else { None })
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.nums[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sieve_test() {
        let sieve = Sieve::new(100);
        let primes: Vec<usize> = sieve.primes().collect();
        assert_eq!(
            primes,
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );

        for n in 0..100 {
            if primes.contains(&n) {
                assert!(sieve.is_prime(n));
            } else {
                assert!(!sieve.is_prime(n));
            }
        }
    }
}
