// proj_02_file_parser — parse a CSV file, print a summary or emit JSON.
// Examples:
//   cargo run --bin proj_02_file_parser -- data/people.csv
//   cargo run --bin proj_02_file_parser -- data/people.csv --json
mod model;

use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Minimal hand-rolled arg parsing (clap was shown in proj_01).
    let args: Vec<String> = env::args().skip(1).collect();
    let path = args
        .first()
        .map(PathBuf::from)
        .context("usage: proj_02_file_parser <file.csv> [--json]")?;
    let as_json = args.iter().any(|a| a == "--json");

    let people = model::read_csv(&path)?;

    if as_json {
        println!("{}", model::to_json(&people)?);
    } else {
        let s = model::summarize(&people);
        println!("records: {}", s.count);
        println!("avg age: {:.1}", s.avg_age);
        println!("oldest : {}", s.oldest.unwrap_or_else(|| "-".into()));
    }
    Ok(())
}
