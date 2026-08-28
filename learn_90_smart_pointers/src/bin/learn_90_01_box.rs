// learn_90_01_box — Box<T>: single-owner heap allocation.
// Run: cargo run --bin learn_90_01_box
// Use Box for large data or recursive types (size unknown at compile time).
#[derive(Debug)]
#[allow(dead_code)] // fields are read only via Debug; kept for the teaching example
enum List {
    Cons(i32, Box<List>), // recursive: Box gives it a known size (a pointer)
    Nil,
}
use List::{Cons, Nil};

fn main() {
    let boxed = Box::new(5); // 5 lives on the heap
    println!("boxed = {boxed}"); // auto-deref

    // A recursive linked list, only possible via Box.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {list:?}");
}
