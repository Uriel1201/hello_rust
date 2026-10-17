use rand::Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = match args[0].parse() {
      Ok(n) => n;
      Err(_) => {
          eprintln!("Error: se esperaba un entero");
          return;
      };
    //let trials: usize = 
}
