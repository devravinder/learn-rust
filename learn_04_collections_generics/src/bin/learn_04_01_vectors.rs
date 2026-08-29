// learn_04_01_vectors — Vec<T>, a growable heap array.
// Run: cargo run --bin learn_04_01_vectors
// Like ArrayList / JS array. Same-type elements, indexable, iterable.
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let v2 = vec![1, 2, 3]; // vec! macro literal
    println!("{v:?} {v2:?}");

    // Indexing panics if out of range; get() returns Option (safe).
    println!("v[0] = {}", v[0]);
    println!("v.get(99) = {:?}", v.get(99));

    // Iterate (borrow), mutate in place, and consume.
    for n in &v {
        print!("{n} ");
    }
    println!();
    for n in &mut v { // &  → borrow / get a reference
        *n += 1; // deref to modify // *  → dereference / access the value behind a reference
    }
    println!("after +1: {v:?}");

    println!("sum = {}", v.iter().sum::<i32>());
    println!("popped = {:?}", v.pop()); // Option
}
