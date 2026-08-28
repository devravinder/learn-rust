// learn_60_03_thiserror — custom error types with #[derive(Error)].
// Run: cargo run --bin learn_60_03_thiserror
// thiserror is for LIBRARIES: define a precise error enum with messages + From.
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("empty input")]
    Empty,
    #[error("value out of range: {0}")]
    OutOfRange(i32),
    #[error("parse failed")]
    Parse(#[from] std::num::ParseIntError), // auto From: `?` converts for us
}

// `?` on parse() converts ParseIntError -> ConfigError::Parse via #[from].
fn parse_port(s: &str) -> Result<i32, ConfigError> {
    if s.is_empty() {
        return Err(ConfigError::Empty);
    }
    let port: i32 = s.parse()?;
    if !(1..=65535).contains(&port) {
        return Err(ConfigError::OutOfRange(port));
    }
    Ok(port)
}

fn main() {
    for input in ["8080", "", "99999", "abc"] {
        match parse_port(input) {
            Ok(p) => println!("{input:?} -> port {p}"),
            Err(e) => println!("{input:?} -> error: {e}"),
        }
    }
}
