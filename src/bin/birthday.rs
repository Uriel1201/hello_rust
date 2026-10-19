// Name: Dante1201 (Carlos Uriel García)
//  Id:   1201

// Write a program birthday.rs that takes two integer command-line arguments n 
// and trials and performs the following experiment, trials times:
 
// - Choose a birthday for the next person, uniformly at random between 0 and n−1.
// - Have that person enter the room.
// - If that person shares a birthday with someone else in the room, stop; otherwise repeat.
 
// In each experiment, count the number of people that enter the room. 
// Print a table that summarizes the results (the count i, the number of times that 
// exactly i people enter the room, and the fraction of times that i or fewer people enter the room) 
// for each possible value of i from 1 until the fraction reaches (or exceeds) 50%.
 
// ~/hello-rust> cargo build --release --bin birthday
 
// ~/hello-rust> ./target/release/birthday 365 1000000
// 0  0  0
// 1  0  0
// 2  2782  0.002782
// 3  5483  0.008265
// 4  8079  0.016343999999999997
// 5  10690  0.027033999999999996
// 6  13113  0.040146999999999995
// 7  15901  0.056047999999999994
// 8  18433  0.07448099999999999
// 9  20214  0.09469499999999999
// 10  22497  0.11719199999999999
// 11  24213  0.141405
// 12  26189  0.167594
// 13  27314  0.194908
// 14  28436  0.223344
// 15  29559  0.252903
// 16  30642  0.283545
// 17  31469  0.315014
// 18  31759  0.346773
// 19  32154  0.378927
// 20  32396  0.411323
// 21  32384  0.443707
// 22  32054  0.475761
// 23  31821  0.507582

use rand::RngExt;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
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
