// learn_13_07_derive_and_builtin — derive (procedural) macros + built-ins.
// Run: cargo run --bin learn_13_07_derive_and_builtin
// So far we've used DECLARATIVE macros (macro_rules!, what Rust by Example
// covers). The OTHER family is PROCEDURAL macros: actual Rust code that runs at
// compile time and transforms a token stream. #[derive(...)] is one kind.
// Writing your own proc-macro needs a separate proc-macro crate (noted below).

/*
 Macros are two types

 DECLARATIVE Macros ( dec-macro ):-
   - macros with rules ( macro_rules! )


 PROCEDURAL Macros ( proc-macro ):-
   - macros that run at compile time ( and transforms a token stream )
   - eg: 
      - built-in: #[derive(Debug)]  
      - our custom: #[drive(Hello)]

     Note:- The word `derive` in #[derive(Hello)] is the compiler's built-in attribute (dispatcher ) that invokes the proc-macro — it is NOT the macro name.


*/

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

    // A few built-in macros 
    let s = format!("{}-{}", a.name, a.retries); // build a String
    println!("format!: {s}");
    println!("vec!: {:?}", vec![1, 2, 3]);
    assert_eq!(2 + 2, 4); // panics if not equal (great in tests)
    println!("assert passed");
}
