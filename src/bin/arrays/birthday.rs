use rand::Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = match args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: was expected an Integer");
            return;
        }
    };
    let trials: usize = match args[1].parse() {
        Ok(trials) => trials,
        Err(_) => {
            eprintln!("Error: was expected an Integer");
            return;
        }
    };    
}
