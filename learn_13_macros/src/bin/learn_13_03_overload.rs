// learn_13_03_overload — a macro with multiple rules (custom syntax).
// Run: cargo run --bin learn_13_03_overload

/*

Macros Overloading:-
 - A macro can have several rules to accept different combinations of arguments. 
 - macro_rules work similarly to a match block ( top-to-bottom )
 - the macro_rule pattern can include arbitrary tokens (like the words `and` / `or`), not just $captures
    - this is how we build custom custom, non-function-like syntax

 - we use semi-colon (;) Aas delimeter to seperate patterns




*/

macro_rules! test {
    // matches:  <expr> ; and <expr>
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // matches:  <expr> ; or <expr>
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
