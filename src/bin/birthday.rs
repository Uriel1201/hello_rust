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
 
// ~/hello_rust> cargo build --release --bin birthday
 
// ~/hello_rust> ./target/release/birthday 365 1000000
// 0, 0, 0
// 1, 0, 0
// 2, 2796, 0.002796
// 3, 5488, 0.008284
// 4, 8209, 0.016493
// 5, 10698, 0.027191
// 6, 13551, 0.040742
// 7, 15700, 0.056442
// 8, 18135, 0.074577
// 9, 20419, 0.094996
// 10, 22189, 0.117185
// 11, 24095, 0.14128
// 12, 25942, 0.16722199999999998
// 13, 27454, 0.194676
// 14, 28458, 0.223134
// 15, 29795, 0.252929
// 16, 31152, 0.28408100000000003
// 17, 31289, 0.31537000000000004
// 18, 31660, 0.34703000000000006
// 19, 32280, 0.37931000000000004
// 20, 32267, 0.411577
// 21, 32195, 0.44377200000000006
// 22, 31599, 0.47537100000000004
// 23, 31423, 0.5067940000000001

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
        println!("{}, {}, {}", k, count, f);
        if f >= 0.5 {
            break;
        }
    }
    ExitCode::SUCCESS
}
