// learn_60_01_result — Result<T, E> and the ? operator.
// Run: cargo run --bin learn_60_01_result
// Result { Ok(T), Err(E) } makes errors explicit in the type (vs try/catch).
// `?` returns early with the Err, otherwise unwraps the Ok. Like `throw` propagation.
use std::num::ParseIntError;

// Returns Result; `?` propagates parse errors up to the caller.
fn double_str(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?; // early-return Err on failure
    Ok(n * 2)
}

fn main() {
    match double_str("21") {
        Ok(v) => println!("ok: {v}"),
        Err(e) => println!("err: {e}"),
    }
    match double_str("abc") {
        Ok(v) => println!("ok: {v}"),
        Err(e) => println!("err: {e}"),
    }

    // Combinators instead of match:
    println!("unwrap_or: {}", double_str("nope").unwrap_or(-1));
    println!("is_ok: {}", double_str("5").is_ok());
}
