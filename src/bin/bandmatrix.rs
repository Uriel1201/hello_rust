// Name: Dante1201 (Carlos Uriel García)
// Id:   1201
 
// Write a program bandmatrix.rs that takes two integer 
// command-line arguments n and width and prints an n-by-n pattern, 
// with a zero (0) for each element whose distance from the main diagnonal is 
// strictly more than width, and an asterisk (*) for each entry that is not. 
// Here, distance means the minimum number of cells you have to move 
// (either left, right, up, or down) to reach the main diagonal.

// ~/hello_rust> cargo build --release --bin bandmatrix
 
// ~/hello_rust> ./target/release/bandmatrix 15 4
// *  *  *  *  *  0  0  0  0  0  0  0  0  0  0  
// *  *  *  *  *  *  0  0  0  0  0  0  0  0  0  
// *  *  *  *  *  *  *  0  0  0  0  0  0  0  0  
// *  *  *  *  *  *  *  *  0  0  0  0  0  0  0  
// *  *  *  *  *  *  *  *  *  0  0  0  0  0  0  
// 0  *  *  *  *  *  *  *  *  *  0  0  0  0  0  
// 0  0  *  *  *  *  *  *  *  *  *  0  0  0  0  
// 0  0  0  *  *  *  *  *  *  *  *  *  0  0  0  
// 0  0  0  0  *  *  *  *  *  *  *  *  *  0  0  
// 0  0  0  0  0  *  *  *  *  *  *  *  *  *  0  
// 0  0  0  0  0  0  *  *  *  *  *  *  *  *  *  
// 0  0  0  0  0  0  0  *  *  *  *  *  *  *  *  
// 0  0  0  0  0  0  0  0  *  *  *  *  *  *  *  
// 0  0  0  0  0  0  0  0  0  *  *  *  *  *  *  
// 0  0  0  0  0  0  0  0  0  0  *  *  *  *  *  

use std::process::ExitCode;
fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut params: Vec<usize> = Vec::with_capacity(2);
    while let Some(nextarg) = args.next() {
        let value: usize = match nextarg.parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: invalid argument {e}");
                return ExitCode::FAILURE;
            }
        };
        params.push(value);
    }
    if params.len() != 2 {
        eprintln!("Usage: n, width");
        return ExitCode::FAILURE;
    }
    for i in 0..params[0] {
        for j in 0..params[0] {
            if i > j + params[1] || j > i + params[1] {
                print!("0  ");
            } else {
                print!("*  ");
            }
        }
        println!();
    }
    println!();
    ExitCode::SUCCESS
}
