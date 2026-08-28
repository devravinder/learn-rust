// learn_01_02_variables — let vs let mut.
// Run: cargo run --bin learn_01_02_variables
// let x   -> immutable (like `const` in JS)
// let mut -> mutable   (like `let` in JS)
fn main() {
    let name = "Ravinder"; // immutable
    let mut last_name = "Reddy"; // mutable
    println!("Full name: {name} {last_name}");

    last_name = "Reddy Kothabad";
    println!("Full name: {name} {last_name}");

    // Positional {} vs captured {name}; only {} can hold an expression.
    println!("lowercased: {name} {}", last_name.to_lowercase());
}
