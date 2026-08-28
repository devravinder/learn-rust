// learn_13_02_derive_and_builtin — derive macros + common built-in macros.
// Run: cargo run --bin learn_13_02_derive_and_builtin
// #[derive(...)] are PROCEDURAL macros that generate trait impls at compile time.
// Writing your own proc-macros needs a separate proc-macro crate (noted below).

#[derive(Debug, Clone, PartialEq, Default)]
struct Config {
    name: String,
    retries: u32,
}

fn main() {
    // Derived impls in action.
    let a = Config { name: "svc".into(), retries: 3 };
    let b = a.clone();
    println!("debug: {a:?}");
    println!("clone eq: {}", a == b);
    println!("default: {:?}", Config::default());

    // A few built-in macros you'll use often:
    let s = format!("{}-{}", a.name, a.retries); // build a String
    println!("format!: {s}");
    println!("vec!: {:?}", vec![1, 2, 3]);
    assert_eq!(2 + 2, 4); // panics if not equal (great in tests)
    println!("assert passed");
    // dbg!(&a);   // prints file:line + value; handy while debugging
    // eprintln!() // like println! but to stderr

    // NOTE: to write your OWN #[derive(MyTrait)], create a crate with
    // [lib] proc-macro = true using syn + quote. That's beyond this intro,
    // but this is the category #[derive(Debug)] etc. belong to.
}
