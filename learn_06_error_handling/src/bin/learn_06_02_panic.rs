// learn_06_02_panic — panic! is for unrecoverable bugs, not normal errors.
// Run: cargo run --bin learn_06_02_panic
// panic! aborts the current thread (like an unchecked exception you don't catch).
// Prefer Result for expected failures; reserve panic for "this should never happen".
fn main() {
    // unwrap()/expect() panic on None/Err — fine for examples, risky in real code.
    let ok: Result<i32, &str> = Ok(5);
    println!("expect ok: {}", ok.expect("should be Ok"));

    // Guard against a known bad state explicitly.
    let denom = 2;
    if denom == 0 {
        panic!("division by zero"); // unreachable here, shown for illustration
    }
    println!("10 / {denom} = {}", 10 / denom);

    // Recoverable path is preferred — return Result instead of panicking.
    println!("safe_div(10, 0) = {:?}", safe_div(10, 0));
    println!("safe_div(10, 5) = {:?}", safe_div(10, 5));
}

fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
