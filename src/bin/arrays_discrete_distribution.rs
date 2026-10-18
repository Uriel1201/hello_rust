//  Name: Dante1201 (Carlos Uriel García)
//  Id:   1201
 
//  Write a program arrays_discrete_distribution.rs that takes an integer command-line 
//  argument m, followed by a sequence of positive integer command-line arguments a1,a2,…,an
//  and prints m random indices (separated by whitespace), choosing each index i
//  with probability proportional to ai.
 
//  ~/hello-rust> cargo build --release --bin arrays_discrete_distribution
 
//  ~/hello-rust> ./target/release/arrays_discrete_distribution 30 10 10 10 20 40
//  1  5  5  4  3  2  2  4  4  5  4  3  4  5  4  4  3  5  5  5  5  5  5  5  5  5  5  1  5  4  
 

public class DiscreteDistribution {
   
    public static void main(String[] args) {
        
        int n = args.length;
        int m = Integer.parseInt(args[0]);
        
        int[] a = new int[n - 1];
        for (int i = 0; i < n - 1; i++) {
            a[i] = Integer.parseInt(args[i + 1]);
        }
        
        int[] s = new int[n];
        for (int i = 1; i < n; i++) {
            s[i] = s[i - 1] + a[i - 1];
        }
        
        for (int k = 0; k < m; k++) {
            int p = (int) (Math.random() * s[n - 1]);
           
            for (int j = 1; j < n; j++) {
                if (s[j] > p && s[j - 1] <= p)
                    System.out.print(j + "  ");
            }
        }
    }
}

use rand::RngExt;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(argsnext) = args.next() else {eprintln!("Error: Missing argument"), return ExitCode::FAILURE};
    let m: usize = match argsnext.parse() {
        Ok(m) => m,
        Err(_) => {
            eprintln!("Error: was expected an Integer");
            return ExitCode::FAILURE;
        }
    };
    ExitCode::SUCCESS
}
