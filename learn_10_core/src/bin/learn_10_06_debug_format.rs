// learn_10_06_debug_format — Display vs Debug.
// Run: cargo run --bin learn_10_06_debug_format
// {}   -> Display (user-facing)
// {:?}  -> Debug (dev-facing), {:#?} pretty. Derive with #[derive(Debug)].
fn main() {
    let data = [1, 2, 3, 4, 5];
    println!("{data:?}"); // arrays have Debug, not Display
    println!("{data:#?}"); // pretty
    println!("{}", 5); // Display
}
