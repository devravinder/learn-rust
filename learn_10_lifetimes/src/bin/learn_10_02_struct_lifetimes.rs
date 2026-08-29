// learn_10_02_struct_lifetimes — structs that hold references.
// Run: cargo run --bin learn_10_02_struct_lifetimes
// A struct holding a reference must declare a lifetime: the struct can't outlive
// the data it borrows.
struct Excerpt<'a> {
    part: &'a str, // borrows from some owner that must outlive the struct
}

impl<'a> Excerpt<'a> {
    fn part(&self) -> &str {
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ravinder. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let e = Excerpt { part: first_sentence }; // e borrows from `novel`
    println!("excerpt: {}", e.part());
    // `e` must not outlive `novel` — the compiler enforces it.
}
