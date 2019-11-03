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
