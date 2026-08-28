// learn_20_04_stack_heap — where values live and why ownership exists.
// Run: cargo run --bin learn_20_04_stack_heap
// Stack: fixed-size, fast, LIFO (i32, bool, arrays). Heap: dynamic-size (String, Vec).
// Ownership tracks heap allocations so they're freed exactly once, no GC needed.
fn main() {
    // Stack: size known at compile time, copied cheaply.
    let x = 42; // on the stack
    let arr = [1, 2, 3]; // fixed array, on the stack
    println!("stack: {x} {arr:?}");

    // Heap: String owns a heap buffer; the String struct (ptr/len/cap) is on stack.
    let mut s = String::from("hi");
    s.push_str(" there"); // may reallocate on the heap as it grows
    println!("heap: '{s}' len={} cap={}", s.len(), s.capacity());

    // When `s` goes out of scope, its heap buffer is freed automatically (drop).
    // No manual free, no garbage collector — determined by ownership.
}
