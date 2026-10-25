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

use rand::RngExt;
use arrow_array::{ArrayRef, Float64Array, RecordBatch, UInt32Array};
use arrow_ipc::writer::StreamWriter;
use arrow_schema::{DataType, Field, Schema};
use std::error::Error;
use std::sync::Arc;

fn arg_parse() -> Result<(usize, usize), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        return Err("Usage birthday <n> <trials>".to_string());
    }
    let n: usize          = args[0].parse().map_err(|_| "Expected an integer")?;
    let trials: usize     = args[1].parse().map_err(|_| "Expected an integer".to_string())?;
    if n == 0 || trials == 0 {
        return Err("n and trials must be greater than 0".to_string());
    }
    Ok((n, trials))
}

fn birthday_simulation(n: usize, trials: usize) -> Vec<u32> {
    let mut s   = vec![0u32; n + 2];
    let mut rng = rand::rng();
    for _ in 0..trials {
        let mut birthday = vec![false; n];
        let mut count    = 0;
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
    s
}

fn main() -> Result<(), Box<dyn Error>> {
    let (n, trials) = arg_parse()?;
    let simulation  = birthday_simulation(n, trials);
    let mut kvec    = Vec::new();
    let mut svec    = Vec::new();
    let mut pvec    = Vec::new();
    let mut f       = 0.0;
    let t           = trials as f64;
    for (k, value) in simulation.into_iter().enumerate() {
        f += value as f64 / t;
        kvec.push(k as u32);
        svec.push(value);
        pvec.push(f);
        if f >= 0.5 {
            break;
        }
    }
    let schema = Arc::new(Schema::new(vec![
        Field::new("k",         DataType::UInt32,  false),
        Field::new("frequency", DataType::UInt32,  false),
        Field::new("p_k",       DataType::Float64, false),
    ]));
    let columns: Vec<ArrayRef> = vec![Arc::new(UInt32Array::from(kvec)),
        Arc::new(UInt32Array::from(svec)),
        Arc::new(Float64Array::from(pvec))
    ];
    let batch = RecordBatch::try_new(schema.clone(), columns)?;
    let mut w = StreamWriter::try_new(std::io::stdout().lock(), &schema)?;
    w.write(&batch)?;
    w.finish()?;
    Ok(())
}
