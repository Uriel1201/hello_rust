// Name: Dante1201 (Carlos Uriel García)
// Id:   1201
 
// Write a program random_walkers.rs that takes two integer command-line arguments r and trials. 
// In each of trials independent experiments, simulate a random walk 
// until the random walker is at Manhattan distance r from the starting point. 
// Print the average number of steps.

// ~/hello-rust> cargo build --release --bin random_walkers

// ~/hello-rust> ./target/release/random_walkers 12 1000000

use rand::RngExt;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut params: Vec<u64> = Vec::with_capacity(2);
    while let Some(nextarg) = args.next() {
        let value: u64 = match nextarg.parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: invalid argument {e}");
                return ExitCode::FAILURE;
            }
        };
        params.push(value);
    }
    let &[manhattan, trials] = params.as_slice() else {
        eprintln!("Usage: Manhattan_distance, trials");
        return ExitCode::FAILURE;
    };
    if trials == 0 {
        eprintln!("Error: trials must be positive");
        return ExitCode::FAILURE;
    }
    let mut sum: u64 = 0;
    for _ in 0..trials {
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut distance: u64 = 0;
        let mut steps: u64 = 0;
        let mut rng = rand::rng();
        while distance < manhattan {
            let x: f64 = rng.random();
            if x < 0.25 {
                i += 1;
            } else if x < 0.5 {
                i -= 1;
            } else if x < 0.75 {
                j += 1;
            } else {
                j -= 1;
            }
            distance = i.unsigned_abs() + j.unsigned_abs();
            steps += 1;
        }
        sum += steps;
    }
    let f = sum as f64 / trials as f64;
    println!("Average Steps: {}", f);
    ExitCode::SUCCESS
}
