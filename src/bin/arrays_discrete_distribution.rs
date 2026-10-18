//  Name: Dante1201 (Carlos Uriel García)
//  Id:   1201
 
//  Write a program arrays_discrete_distribution.rs that takes an integer command-line 
//  argument m, followed by a sequence of positive integer command-line arguments a1,a2,…,an
//  and prints m random indices (separated by whitespace), choosing each index i
//  with probability proportional to ai.
 
//  ~/hello-rust> cargo build --release --bin arrays_discrete_distribution
 
//  ~/hello-rust> ./target/release/arrays_discrete_distribution 30 10 10 10 20 40
//  1  5  5  4  3  2  2  4  4  5  4  3  4  5  4  4  3  5  5  5  5  5  5  5  5  5  5  1  5  4  
 
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
    let m: u64 = values.remove(0);
    let n = values.len() + 1;
    let mut s: Vec<u64> = Vec::with_capacity(n);
    s.push(0);
    let mut current: u64 = 0;
    for value in values {
        let next: u64 = current + value;
        s.push(next);
        current = next;
    }
    let mut rng = rand::rng();
    for _ in 0..m {
        let p = rng.random_range(0..*s.last().unwrap());
        for j in 1..n {
            if s[j] > p && s[j - 1] <= p {
                print!("{j}  ");
            }
        }
    }
    ExitCode::SUCCESS
}
