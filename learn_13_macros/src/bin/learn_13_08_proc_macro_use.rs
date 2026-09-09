// learn_13_08_proc_macro_use — USING our own #[derive(Hello)] proc-macro.
// Run: cargo run --bin learn_13_08_proc_macro_use


/*
 - The macro itself lives in a SEPARATE crate (learn_13_proc_macro)

 - because proc-macro crates compile into a compiler plugin and can't sit in src/bin/.
 - Here we just import it and put it to work — exactly like #[derive(Debug)],

Reminder: in #[derive(Hello)], `derive` is the compiler's built-in attribute and `Hello` is our macro's name.


See ../../learn_13_proc_macro/src/lib.rs for the 3-step PARSE/INSPECT/BUILD

*/

// Bring the derive macro into scope.
use learn_13_proc_macro::Hello;


/*

At compile time, #[derive(Hello)] generates:
    impl Dog { 
      fn hello(&self) { 
        println!("Hello from {}!", "Dog"); 
        } 
    }

*/

#[derive(Hello)]
struct Dog;

#[derive(Hello)]
struct Robot;

fn main() {
    let d = Dog;
    let r = Robot;

    // These methods only exist because the derive macro generated them.
    d.hello(); // Hello from Dog!
    r.hello(); // Hello from Robot!
}
