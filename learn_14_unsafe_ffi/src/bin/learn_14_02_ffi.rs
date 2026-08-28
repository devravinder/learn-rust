// learn_14_02_ffi — call C functions from Rust (Foreign Function Interface).
// Run: cargo run --bin learn_14_02_ffi
// `extern "C"` declares functions with the C ABI. Calling them is `unsafe`
// because the compiler can't verify the foreign code. This is how crates wrap
// C libraries — and conceptually how Tauri bridges to native APIs.

// Declare C standard library functions we want to call.
unsafe extern "C" {
    fn abs(input: i32) -> i32; // from the C runtime
    fn sqrt(x: f64) -> f64;
}

fn main() {
    // Calls into C must be inside `unsafe`.
    unsafe {
        println!("C abs(-42) = {}", abs(-42));
        println!("C sqrt(2.0) = {:.5}", sqrt(2.0));
    }

    // In real code you'd wrap these in a SAFE Rust function that upholds the
    // C function's contract, so callers never touch `unsafe` themselves:
    println!("safe wrapper abs(-7) = {}", safe_abs(-7));
}

// A safe wrapper: the unsafe is contained and justified here.
fn safe_abs(n: i32) -> i32 {
    unsafe { abs(n) }
}
