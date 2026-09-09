// concept_14_01_macro_rules — the basics of macro_rules!.
// Run: cargo run --bin concept_14_01_macro_rules

/*
 ref: https://doc.rust-lang.org/stable/rust-by-example/macros.html


 Macro:-
  - is code that writes code. 
  - runs at compile time on SYNTAX — it expands into more Rust source before type checking.
  - it's identiifier is - prefixex with `macro_rules!` before it's name/identifier

  - Macro rules:-
    - macro_rules! works like a `match`, but it matches on SYNTAX, not values.
    - Each rule is: (pattern) => { expansion };


 Uses:-
  - Don’t repeat yourself - to avoid repeating code ( dynamic code based on types )
  - DSL ( Domain-specific languages ) - to write code with special syntax for a specific purpose
  - Variadic interfaces - to define an interface that takes a variable number of arguments. eg: println!

*/

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


// The simplest possible macro: `()` matches "no arguments".
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// Macros can also match literal tokens as part of the pattern.
macro_rules! greet {
    // matches the exact word `world`
    (world) => {
        println!("Hello, world!");
    };
    // matches the exact word `rust`
    (rust) => {
        println!("Hello, Rust!");
    };
}

fn main() {
    // `say_hello!()` expands into `println!("Hello!");` at compile time.
    say_hello!();

    greet!(world);
    greet!(rust);
}
