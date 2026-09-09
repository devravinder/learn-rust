// learn_13_02_designators — capturing syntax with $name:designator.
// Run: cargo run --bin learn_13_02_designators

/*

 ref: https://doc.rust-lang.org/stable/rust-by-example/macros/designators.html
 ref: https://doc.rust-lang.org/reference/macros-by-example.html

 Designators-
  - is a syntax category that tells the macro what kind of syntax to capture ( or match )
  - is the part after the colon in a macro capture
    eg:-
     macro_rules! say_hello {
        ($name:ident) => {
            println!("Hello! {}", $name);
        };
    }

    Here:-
    - $name -> meta variable
    - ident -> designator
        - this `ident` tells Rust that $name must be an identifier. Eg: Ram, Ravi


    hello!(Ram);



 - Available Designators (Categories):-
    - `block`
    - `expr` is used for expressions
    - `ident` is used for variable/function names
    - `item`
    - `literal` is used for literal constants
    - `pat` (pattern)
    - `path`
    - `stmt` (statement)
    - `tt` (token tree)
    - `ty` (type)
    - `vis` (visibility qualifier)



 - The arguments of a macro are prefixed by a dollar sign $ and type annotated with a designator:





*/

// `ident` captures a name — here we use it to deifne a function.
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            // stringify! turns the captured token into a string literal.
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// Generate two real functions at compile time.
create_function!(foo);
create_function!(bar);

// `expr` captures a whole expression.
macro_rules! print_result {
    ($expression:expr) => {
        // print the expression's source text AND its evaluated value.
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}


fn main() {
    foo(); // defined by the macro above
    bar();

    print_result!(1u32 + 1);

    
    // a block is also an expression, so this works too:
    print_result!({
        let x = 1u32;
        x * x
    });

}
