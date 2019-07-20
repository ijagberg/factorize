pub fn trial_division(mut n: u128) -> Vec<u128> {
    let mut factors: Vec<u128> = Vec::new();
    if n == 0 {
        return factors;
    }
    
    let mut candidates = std::iter::once(2).chain((3..).step_by(2));
    
    while n > 1 {
        let candidate = candidates.next().unwrap();
        while n % candidate == 0 {
            factors.push(candidate);
            n /= candidate;
        }
    }
    
    factors
}
