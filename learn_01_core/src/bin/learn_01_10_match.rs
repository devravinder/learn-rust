// learn_01_10_match — exhaustive pattern matching.
// Run: cargo run --bin learn_01_10_match
// Like switch, but exhaustive and an expression (returns a value).
use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    let secret = rand::rng().random_range(1..=10);
    println!("secret = {secret}");

    println!("Guess 1 to 10:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("read failed");
    let guess: i32 = guess.trim().parse().expect("invalid number");

    let message = match guess.cmp(&secret) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    };
    println!("{message}");
}
