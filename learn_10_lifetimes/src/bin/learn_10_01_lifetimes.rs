// learn_10_01_lifetimes — lifetime annotations on references.
// Run: cargo run --bin learn_10_01_lifetimes
// Lifetimes tell the compiler how long references are valid. They don't change
// runtime behavior — they let the borrow checker prove no dangling references.
// 'a is a generic lifetime parameter (like <T>, but for reference validity).

/*
 lifetimes tells rust compiler ...how long a value is valid
*/

// The returned &str lives as long as the SHORTER of the two inputs ('a).
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(&s1, &s2);
        println!("longest = {result}"); // used while both are alive
    }
    // Using `result` here would be an error if it pointed into s2 (dropped).

    // Most code needs NO explicit lifetimes thanks to elision:
    println!("first word: {}", first_word("hello world"));
}

// Elided: compiler infers the output ties to the input, so no 'a needed.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
