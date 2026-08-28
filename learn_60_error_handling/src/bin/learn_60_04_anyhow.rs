// learn_60_04_anyhow — easy error handling for applications.
// Run: cargo run --bin learn_60_04_anyhow
// anyhow::Result<T> = Result<T, anyhow::Error>: any error type works with `?`.
// Use in apps/main; use thiserror in libraries you publish.
use anyhow::{Context, Result, bail};

fn load_setting(raw: &str) -> Result<i32> {
    if raw.is_empty() {
        bail!("no setting provided"); // early return with an ad-hoc error
    }
    // .context() attaches a helpful message to whatever error `?` produces.
    let value: i32 = raw
        .trim()
        .parse()
        .context("setting must be an integer")?;
    Ok(value)
}

fn main() -> Result<()> {
    println!("ok: {}", load_setting("42")?);

    // Show the error+context chain without crashing main.
    if let Err(e) = load_setting("oops") {
        println!("error: {e:#}"); // {:#} prints the full context chain
    }
    Ok(())
}
