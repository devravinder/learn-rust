// learn_13_08_proc_macro_use — USING our own proc-macros (all 3 kinds).
// Run: cargo run --bin learn_13_08_proc_macro_use
//
// The macros live in a SEPARATE crate (learn_13_proc_macro) because proc-macro
// crates compile into a compiler plugin and can't sit in src/bin/.
//
// Reminder: in #[derive(Hello)], `derive` is the compiler's built-in attribute
// and `Hello` is our macro's name.
//
// See ../../learn_13_proc_macro/src/lib.rs for the implementations.

// Bring all three macros into scope.
use learn_13_proc_macro::{Hello, log_call, make_answer};

/* 1. DERIVE — #[derive(Hello)] generates: impl Dog { fn hello(&self) {...} } */
#[derive(Hello)]
struct Dog;

#[derive(Hello)]
struct Robot;

/* 2. ATTRIBUTE — #[log_call] rewrites greet() to log before its body runs */
#[log_call]
fn greet() {
    println!("hello from greet()");
}

/* 3. FUNCTION-LIKE — make_answer!() defines `fn answer() -> u32 { 42 }` */
make_answer!();

fn main() {
    // derive: methods that only exist because the macro generated them
    Dog.hello(); // Hello from Dog!
    Robot.hello(); // Hello from Robot!

    // attribute: prints "--> calling greet" then the body
    greet();

    // function-like: call the fn the macro defined
    println!("answer() = {}", answer()); // 42
}
