// learn_10_07_strings — String vs &str + stdin.
// Run: cargo run --bin learn_10_07_strings
// String = owned, growable (heap). &str = borrowed, read-only view.
use std::io;

fn main() {
    let name = String::from("ravinder"); // &str -> String
    println!("name: {name}");

    let mut full_name = String::new(); // empty growable buffer
    println!("Type your name:");

    // read_line appends input (incl. newline); returns Result.
    io::stdin()
        .read_line(&mut full_name)
        .expect("Error reading input");

    println!("Your name: {}", full_name.trim());
}
