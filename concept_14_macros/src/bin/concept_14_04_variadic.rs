// concept_14_04_variadic — variadic macros with repetition $( ... )*.
// Run: cargo run --bin concept_14_04_variadic

/*

 Repeat ( Variadic Trait ):-
  - macro can take arbitrary number of argumemets
      - normal functions cannot do this. This is how vec! works.


  -  repeat_notations:-
     - * = zero or more  ( any )
     - + = one or more   ( required.... minimum one )
     - ? = zero or one
    
   - syntax:-   $( ... )<seperator><repeat_notation>
           
       - the separator (e.g. a comma) goes right before the repeat_notation
       - the matcher surrounded with $(...)

    - eg: $($y:expr),+ 
       -  will match one or more expression, separated by commas


  - Note:- the semicolon is optional on the last case ( match rule )
  

*/

// Recursive variadic macro (RBE's find_min! shape): first element + the rest.
macro_rules! find_min {
    // base case: a single expression
    ($x:expr) => ($x);

    

    // recursive case: `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    ) // no semi colon  ( optional )
}

// A mini vec! built from scratch.
macro_rules! my_vec {
    // zero or more comma-separated expressions;
    // trailing comma allowed via $(,)?
    ( $( $x:expr ),* $(,)? ) => {{
        let mut v = Vec::new();
        $( v.push($x); )*  // the push is REPEATED once per matched $x
        v
    }};
}



fn main() {

    println!("find_min!(1)          = {}", find_min!(1));
    println!("find_min!(5, 2, 1 + 3) = {}", find_min!(5, 2, 1 + 3));
    println!("find_min!(2,3,4,1,5)  = {}", find_min!(2, 3, 4, 1, 5));

    
    let v: Vec<i32> = my_vec![1, 2, 3, 4];
    println!("my_vec = {v:?}");

    let v: Vec<i32> = my_vec![1, 2, 3, 4, 5,]; // tailing comma
    println!("my_vec = {v:?}");
}
