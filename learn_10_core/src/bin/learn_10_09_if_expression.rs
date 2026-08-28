// learn_10_09_if_expression — `if` returns a value.
// Run: cargo run --bin learn_10_09_if_expression
// Like a ternary for whole blocks; all branches must be the same type.
use std::io;
use rand::prelude::*;

fn main() {
    let secret = rand::rng().random_range(1..=10); // inclusive of 10
    println!("secret = {secret}");

    println!("Guess 1 to 10:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("read failed");
    let guess: i32 = guess.trim().parse().expect("invalid number");

    let message = if guess == secret {
        "correct"
    } else if guess < secret {
        "low"
    } else {
        "high"
    };
    println!("{message}");
}
