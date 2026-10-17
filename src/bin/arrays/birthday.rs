use rand::RngExt;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = match args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: was expected an Integer");
            return ExitCode::FAILURE;
        }
    };
    let trials: usize = match args[1].parse() {
        Ok(trials) => trials,
        Err(_) => {
            eprintln!("Error: was expected an Integer");
            return ExitCode::FAILURE;
        }
    };
    let mut s = vec![0u64; n + 2];
    let mut rng = rand::rng();
    for _ in 0..trials {
        let mut birthday = vec![false; n];
        let mut count = 0;
        for _ in 0..n + 1 {
            let p = rng.random_range(0..n);
            count += 1;
            if birthday[p] {
                break;
            }
            birthday[p] = true;
        }
        s[count] += 1;
    }
    ExitCode::SUCCESS
}
