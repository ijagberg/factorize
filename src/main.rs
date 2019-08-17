use std::env;
use std::time;
pub mod algorithms;

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
        let factors = algorithms::trial_division(number);
        let mut product: u128 = 1;
        for factor in &factors {
            product *= factor;
        }
        println!("{} => {:?}, took {:?}", number, factors, timer.elapsed());
        assert_eq!(number, product);
    }
}
