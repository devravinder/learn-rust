// learn_10_12_guess_number — mini-project combining the core concepts.
// Run: cargo run --bin learn_10_12_guess_number
// Uses: loop, match on Ordering, parse-with-Result (continue on bad input).
use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    let secret = rand::rng().random_range(1..=10);

    loop {
        println!("Guess 1 to 10:");
        let mut guess = String::new(); // fresh buffer each turn
        io::stdin().read_line(&mut guess).expect("read failed");

        // parse -> Result; retry instead of crashing on bad input
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("bad input: {e}, try again");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
