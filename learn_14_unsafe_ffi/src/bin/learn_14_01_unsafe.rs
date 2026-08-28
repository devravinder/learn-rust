// learn_14_01_unsafe — the `unsafe` keyword: opt out of some compiler checks.
// Run: cargo run --bin learn_14_01_unsafe
// `unsafe` does NOT turn off the borrow checker. It only unlocks 5 abilities,
// mainly: deref raw pointers, call unsafe fns, access/modify static mut, etc.
// You promise the compiler you've upheld the invariants it can't verify.
fn main() {
    let mut num = 5;

    // Raw pointers (*const / *mut) can be created safely...
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // ...but DEREFERENCING them requires unsafe (they may dangle/alias).
    unsafe {
        println!("*r1 = {}", *r1);
        *r2 = 10; // write through a raw pointer
        println!("*r2 = {}", *r2);
    }
    println!("num = {num}");

    // Calling an unsafe fn also needs an unsafe block.
    unsafe {
        dangerous();
    }
}

// Marking a fn `unsafe` means "callers must uphold my safety contract".
unsafe fn dangerous() {
    println!("called an unsafe fn (caller vouched for safety)");
}
