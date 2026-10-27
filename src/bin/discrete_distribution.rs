// Name: Dante1201 (Carlos Uriel García)
// Id:   1201

// Write a program discrete_distribution.rs that takes an integer command-line
// argument m, followed by a sequence of positive integer command-line arguments a1,a2,…,an
// and prints m random indices, choosing each index i
// with probability proportional to ai.

// ~/hello_rust $ cargo build --release --bin discrete_distribution

// ~/hello_rust $ ./target/release/discrete_distribution 5000000 15 10 10 12

use rand::RngExt;
use arrow_array::{ArrayRef, RecordBatch, UInt64Array};
use arrow_ipc::writer::StreamWriter;
use arrow_schema::{DataType, Field, Schema};
use std::error::Error;
use std::sync::Arc;

fn arg_parse() -> Result<(usize, Vec<u64>), String> {
    const USAGE: &str         = "Usage: discrete_distribution m a1 a2 ... an";
    let mut args              = std::env::args().skip(1);
    let Some(arg_0)           = args.next() else {
        return Err(USAGE.to_string());
    };
    let m: usize              = arg_0
        .parse()
        .map_err(|_| "Expected m to be a non-negative integer".to_string())?;
    let mut weights: Vec<u64> = Vec::new();
    while let Some(next_arg)  = args.next() {
        let w: u64 = next_arg
            .parse()
            .map_err(|_| "Expected ai to be a positive integer".to_string())?;
        if w == 0 {
            return Err("Expected ai to be a positive integer".to_string());
        }
        weights.push(w);
    }
    if weights.is_empty() {
        return Err(USAGE.to_string());
    }
    Ok((m, weights))
}

fn cumweights(mut weights: Vec<u64>) -> Result<Vec<u64>, String> {
    for i in 1..weights.len() {
        weights[i] = weights[i]
            .checked_add(weights[i - 1])
            .ok_or("Sum of weights overflows u64")?;
    }
    Ok(weights)
}

fn discrete_distribution(size: usize,
    cum:&[u64],
    rng: &mut impl RngExt
) -> Result<Vec<usize>, String> {
    let mut dis_dist: Vec<usize> = Vec::with_capacity(size);
    let total: u64 = *cum.last().ok_or("empty weights")?;
    for _ in 0..size {
        let p = rng.random_range(1..=total);
        let j = cum.partition_point(|&x| x < p);
        dis_dist.push(j + 1);
    }
    Ok(dis_dist)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (m, weights)  = arg_parse()?;
    let cum_weights   = cumweights(weights)?;
    let mut rng       = rand::rng();
    let sample        = discrete_distribution(m, &cum_weights, &mut rng)?;
    let col: Vec<u64> = sample.into_iter().map(|s| s as u64).collect();
    let schema = Arc::new(Schema::new(vec![
        Field::new("INDEX_SAMPLE", DataType::UInt64,  false),
    ]));
    let columns: Vec<ArrayRef> = vec![Arc::new(UInt64Array::from(col))];
    let batch                  = RecordBatch::try_new(schema.clone(), columns)?;
    let mut w                  = StreamWriter::try_new(std::io::stdout().lock(), &schema)?;
    w.write(&batch)?;
    w.finish()?;
    Ok(())
}
