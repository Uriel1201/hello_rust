// Name: Dante1201 (Carlos Uriel García)
// Id:   1201
 
// Write a program random_walker.rs that takes an integer command-line argument r 
// and simulates the motion of a random walk until the random walker is at Manhattan distance r 
// from the starting point. 
// Print the coordinates at each step of the walk (including the starting and ending points), 
// treating the starting point as (0, 0). 
// Also, print the total number of steps taken.

// ~/hello-rust> cargo build --release --bin random_walker
 
// ~/hello-rust> ./target/release/random_walker 5
// (0, 0)
// (-1, 0)
// (-1, -1)
// (0, -1)
// (0, -2)
// (-1, -2)
// (0, -2)
// (0, -3)
// (0, -4)
// (0, -3)
// (1, -3)
// (0, -3)
// (0, -2)
// (1, -2)
// (0, -2)
// (0, -1)
// (1, -1)
// (2, -1)
// (2, -2)
// (3, -2)
// steps = 19

use rand::RngExt;
use std::process::ExitCode;

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

    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut distance: u64 = 0;
    let mut steps: u64 = 0;
    println!("({}, {})", i, j);
    while distance < manhattan {
        let x: f64 = rand::rng().random();
        if x < 0.25 {
            i += 1;
        } else if x < 0.5 {
            i -= 1;
        } else if x < 0.75 {
            j += 1;
        } else {
            j -= 1;
        }
        println!("({}, {})", i, j);
        distance = i.unsigned_abs() + j.unsigned_abs();
        steps += 1;
    }
    println!("steps = {}", steps);
    ExitCode::SUCCESS
}
