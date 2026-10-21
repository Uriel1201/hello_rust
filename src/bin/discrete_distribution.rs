//  Name: Dante1201 (Carlos Uriel García)
//  Id:   1201
 
//  Write a program discrete_distribution.rs that takes an integer command-line 
//  argument m, followed by a sequence of positive integer command-line arguments a1,a2,…,an
//  and prints m random indices (separated by whitespace), choosing each index i
//  with probability proportional to ai.
 
//  ~/hello-rust> cargo build --release --bin discrete_distribution
 
//  ~/hello-rust> ./target/release/discrete_distribution 50 10 10 12

use rand::RngExt;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut values: Vec<u64> = Vec::new();
    while let Some(nextarg) = args.next() {
        let value: u64 = match nextarg.parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error: was expected an Integer");
                return ExitCode::FAILURE;
            }
        };
        values.push(value);
    }
    if values.len() < 2 {
       eprintln!("Usage: m a1 a2 ... an");
       return ExitCode::FAILURE;
    }
    let m: u64 = values.remove(0);
    let mut s: Vec<u64> = Vec::with_capacity(values.len() + 1);
    s.push(0);
    let mut cumulative: u64 = 0;
    for value in values {
        cumulative += value;
        s.push(cumulative);
    }
    let mut rng = rand::rng();
    for _ in 0..m {
        let p = rng.random_range(0..cumulative);
        let j = s.partition_point(|&x| x <= p);
        print!("{j}  ");
    }
    println!();
    ExitCode::SUCCESS
}
