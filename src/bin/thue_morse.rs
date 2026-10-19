// Name: Dante1201 (Carlos Uriel García)
// Id:   1201

// Write a program thue_morse.rs that takes 
// an integer command-line argument n and prints an n-by-n pattern. 
// Include two space characters between each + and - character.
 
// ~/hello-rust> cargo build --release --bin thue_morse
 
//  ~/hello-rust> ./target/release/thue_morse 15
//  +  -  -  +  -  +  +  -  -  +  +  -  +  -  -  
//  -  +  +  -  +  -  -  +  +  -  -  +  -  +  +  
//  -  +  +  -  +  -  -  +  +  -  -  +  -  +  +  
//  +  -  -  +  -  +  +  -  -  +  +  -  +  -  -  
//  -  +  +  -  +  -  -  +  +  -  -  +  -  +  +  
//  +  -  -  +  -  +  +  -  -  +  +  -  +  -  -  
//  +  -  -  +  -  +  +  -  -  +  +  -  +  -  -  
//  -  +  +  -  +  -  -  +  +  -  -  +  -  +  +  
//  -  +  +  -  +  -  -  +  +  -  -  +  -  +  +  
//  +  -  -  +  -  +  +  -  -  +  +  -  +  -  -  
//  +  -  -  +  -  +  +  -  -  +  +  -  +  -  -  
//  -  +  +  -  +  -  -  +  +  -  -  +  -  +  +  
//  +  -  -  +  -  +  +  -  -  +  +  -  +  -  -  
//  -  +  +  -  +  -  -  +  +  -  -  +  -  +  +  
//  -  +  +  -  +  -  -  +  +  -  -  +  -  +  + 

// I applied this hint p(i) == p(j)  ⇔  p(i ^ j) == 0  ⇔  (i ^ j).count_ones() % 2 == 0
// such that p(x) = x.count_ones() % 2 where p(x) is the value of Thue-Morse in the
// position x, starting the sequence in zero.

use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(text) = std::env::args().nth(1) else {
        eprintln!("Error: missing argument m");
        return ExitCode::FAILURE;
    };

    let n: usize = match text.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: was expected a non-negative Integer");
            return ExitCode::FAILURE;
        }
    };
    for i in 0..n {
        for j in 0..n {
            print!(
                "{}  ",
                if (i ^ j).count_ones() % 2 == 0 {
                    "+"
                } else {
                    "-"
                }
            );
        }
        println!();
    }
    return ExitCode::SUCCESS;
}
