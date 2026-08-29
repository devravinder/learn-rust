// learn_02_01_ownership_move — the ownership rules + move semantics.
// Run: cargo run --bin learn_02_01_ownership_move
// Rules: each value has ONE owner; when the owner goes out of scope, value is dropped.
// Assigning/passing a heap value MOVES ownership (unlike JS reference copy or Java).
fn main() {
    // Copy types (on stack): i32, bool, char... are copied, not moved. // primitives get copied
    let a = 5;
    let b = a; // copy
    println!("copy: a={a} b={b}"); // both valid

    // Heap type (String): assignment MOVES ownership.  // objects get moved
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2; s1 is now invalid
    // println!("{s1}"); // <- compile error: value borrowed after move
    println!("moved: s2={s2}");

    // Passing to a function also moves.
    let s = String::from("world");
    takes_ownership(s); // s moved in; dropped at end of function
    // println!("{s}"); // <- error

    // To keep using it, clone (deep copy) or borrow (next file).
    let original = String::from("keep me");
    let copy = original.clone();
    println!("clone: original={original} copy={copy}");
}

fn takes_ownership(text: String) {
    println!("owned: {text}");
} // `text` dropped here
