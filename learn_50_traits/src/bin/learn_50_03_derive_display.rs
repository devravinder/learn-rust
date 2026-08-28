// learn_50_03_derive_display — derive common traits + implement Display.
// Run: cargo run --bin learn_50_03_derive_display
// #[derive(...)] auto-implements traits. Display is manual (user-facing text).
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// Manual Display -> enables {} and .to_string().
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

fn main() {
    let c = Color { r: 255, g: 128, b: 0 };
    let c2 = c.clone(); // from derive(Clone)

    println!("debug: {c:?}"); // derive(Debug)
    println!("display: {c}"); // manual Display
    println!("equal: {}", c == c2); // derive(PartialEq)
    println!("as string: {}", c.to_string()); // free with Display
}
