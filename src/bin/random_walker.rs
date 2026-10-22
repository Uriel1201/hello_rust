// Name: Dante1201 (Carlos Uriel García)
// Id:   1201
 
// Write a program random_walker.rs that takes an integer command-line argument r 
// and simulates the motion of a random walk until the random walker is at Manhattan distance r 
// from the starting point. 
// Print the coordinates at each step of the walk (including the starting and ending points), 
// treating the starting point as (0, 0). 
// Also, print the total number of steps taken.

// ~/hello_rust> cargo build --release --bin random_walker
 
// ~/hello_rust> ./target/release/random_walker 5

use rand::RngExt;
use std::process::ExitCode;
use std::io::{self, BufWriter, Write};

fn main() -> ExitCode {
    let Some(text) = std::env::args().nth(1) else {
        eprintln!("Error: missing argument");
        return ExitCode::FAILURE;
    };
    let manhattan: u64 = match text.parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: invalid argument {e}");
            return ExitCode::FAILURE;
        }
    };
 
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
 
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut steps: u64 = 0;
    let mut rng = rand::rng();
    writeln!(out, "{} {}", i, j).unwrap();
    while i.unsigned_abs() + j.unsigned_abs() < manhattan {
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
        writeln!(out, "{} {}", i, j).unwrap();
        steps += 1;
    }
    writeln!(out, "steps = {}", steps).unwrap();
    out.flush().unwrap();
    ExitCode::SUCCESS
}
