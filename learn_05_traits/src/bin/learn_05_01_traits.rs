// learn_05_01_traits — traits are like interfaces.
// Run: cargo run --bin learn_05_01_traits
// Define shared behavior; types `impl` it. Methods can have default bodies.
trait Summary {
    fn title(&self) -> String; // required

    fn summarize(&self) -> String {
        // default method — types can use or override it
        format!("(read more about '{}')", self.title())
    }
}

struct Article {
    headline: String,
}
struct Tweet {
    user: String,
    text: String,
}

impl Summary for Article {
    fn title(&self) -> String {
        self.headline.clone()
    }
    // uses default summarize()
}

impl Summary for Tweet {
    fn title(&self) -> String {
        format!("@{}", self.user)
    }
    fn summarize(&self) -> String {
        // override
        format!("@{}: {}", self.user, self.text)
    }
}

fn main() {
    let a = Article { headline: "Rust 2024".into() }; // .info to convert to String ( != str)
    let t = Tweet { user: "ferris".into(), text: "traits rock".into() };
    println!("{}", a.summarize());
    println!("{}", t.summarize());
}
