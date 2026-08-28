// learn_13_01_macro_rules — declarative macros with macro_rules!.
// Run: cargo run --bin learn_13_01_macro_rules
// Macros expand into code at compile time (before type checking). They can take
// a variable number of arguments — something functions cannot.

// A macro that builds a Vec, like a mini vec!.
macro_rules! my_vec {
    // match: zero or more expressions separated by commas
    ( $( $x:expr ),* $(,)? ) => {{
        let mut v = Vec::new();
        $( v.push($x); )*  // repeat the push for each matched $x
        v
    }};
}

// A macro with multiple rules (matches on shape).
macro_rules! max {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => {{
        let a = $a;
        let b = max!($($rest),+); // recursive expansion
        if a > b { a } else { b }
    }};
}

fn main() {
    let v: Vec<i32> = my_vec![1, 2, 3, 4];
    println!("my_vec = {v:?}");

    println!("max = {}", max!(3, 7, 2, 9, 5));
}
