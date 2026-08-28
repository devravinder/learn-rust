// learn_01_04_constants — const (compile-time, always immutable).
// Run: cargo run --bin learn_01_04_constants
// Like `static final` in Java. Type annotation required.
fn main() {
    const DEVELOPER_AGE: u32 = 30;
    const SECONDS_IN_HOUR: u32 = 60 * 60; // const expression
    println!("{DEVELOPER_AGE} {SECONDS_IN_HOUR}");
}
