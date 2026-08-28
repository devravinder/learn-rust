// learn_01_08_functions — declaring & calling functions.
// Run: cargo run --bin learn_01_08_functions
// Param/return types are mandatory. Last expression (no `;`) is the return value.
fn main() {
    greet("Ravinder");
    println!("add(3,4) = {}", add(3, 4));
    println!("classify(-5) = {}", classify(-5));

    let doubled = {
        let base = 21;
        base * 2 // block value (no `;`)
    };
    println!("doubled = {doubled}");
}

fn greet(name: &str) {
    // no return -> unit type ()
    println!("Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b // no `;` = return value
}

fn classify(n: i32) -> &'static str {
    if n < 0 {
        return "negative"; // early return
    }
    if n == 0 { "zero" } else { "positive" }
}
