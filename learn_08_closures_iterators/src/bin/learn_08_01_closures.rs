// learn_08_01_closures — anonymous functions that capture their environment.
// Run: cargo run --bin learn_08_01_closures
// |args| body — like JS arrow functions. They can capture surrounding variables.
fn main() {
    let add = |a: i32, b: i32| a + b; // typed closure
    println!("add = {}", add(2, 3));

    // Capture by reference.
    let factor = 10;
    let scale = |x: i32| x * factor; // borrows `factor`
    println!("scale(5) = {}", scale(5));

    // Capture by move (takes ownership) — needed when returning/spawning.
    let name = String::from("Rust");
    let greet = move || format!("Hello, {name}!");
    println!("{}", greet());

    // Passing a closure to a function (FnMut for mutation).
    let mut count = 0;
    let mut bump = || count += 1;
    bump();
    bump();
    println!("count = {count}");

    // Returning a closure (boxed trait object) / accepting via generics.
    let doubler = make_multiplier(2);
    println!("doubler(21) = {}", doubler(21));
}

// Return an `impl Fn` closure.
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}
