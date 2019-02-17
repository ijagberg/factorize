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
