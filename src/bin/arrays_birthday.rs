//! Name: Dante1201 (Carlos Uriel García)
//!  Id:   1201

//! Write a program Birthday.java that takes two integer command-line arguments n 
//! and trials and performs the following experiment, trials times:
 
//! - Choose a birthday for the next person, uniformly at random between 0 and n−1.
//! - Have that person enter the room.
//! - If that person shares a birthday with someone else in the room, stop; otherwise repeat.
 
//! In each experiment, count the number of people that enter the room. 
//! Print a table that summarizes the results (the count i, the number of times that 
//! exactly i people enter the room, and the fraction of times that i or fewer people enter the room) 
//! for each possible value of i from 1 until the fraction reaches (or exceeds) 50%.
 
//! ~/hello-rust> cargo build --release --bin arrays_birthday
 
//! ~/hello-rust> ./target/release/arrays_birthday 365 1000000

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
    let mut f = 0.0;
    for (k, count) in s.into_iter().enumerate() {
        f += count as f64 / trials as f64;
        println!("{}  {}  {}", k, count, f);
        if f >= 0.5 {
            break;
        }
    }
    ExitCode::SUCCESS
}
