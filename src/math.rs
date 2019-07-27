pub fn trial_division(mut n: u128) -> Vec<u128> {
    let mut factors: Vec<u128> = Vec::new();
    if n == 0 {
        return factors;
    }
    
    let candidates = std::iter::once(2).chain((3..).step_by(2));
    
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
