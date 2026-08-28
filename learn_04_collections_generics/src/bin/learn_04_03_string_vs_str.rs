// learn_04_03_string_vs_str — owned String vs borrowed &str, in practice.
// Run: cargo run --bin learn_04_03_string_vs_str
// Take &str as params (accepts both); build/return String.
fn main() {
    let owned: String = String::from("hello");
    let literal: &str = "world"; // &'static str

    // &String coerces to &str, so this works for both.
    println!("{}", shout(&owned));
    println!("{}", shout(literal));

    // Building strings.
    let mut s = String::new();
    s.push_str("foo");
    s.push('/');
    s += "bar"; // += works with &str
    let joined = format!("{s}/baz"); // format! like a template literal
    println!("{joined}");

    // Split / collect.
    let parts: Vec<&str> = "a,b,c".split(',').collect();
    println!("{parts:?}");
    println!("rejoined = {}", parts.join("-"));
}

fn shout(text: &str) -> String {
    text.to_uppercase()
}
