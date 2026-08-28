// learn_20_02_borrowing — references (&) let you use a value without owning it.
// Run: cargo run --bin learn_20_02_borrowing
// &T = shared/immutable borrow (many allowed). &mut T = exclusive/mutable borrow (one).
// Rule: any number of &T, OR exactly one &mut T, never both at once.
fn main() {
    let s = String::from("hello");
    let len = calc_len(&s); // borrow, don't move
    println!("'{s}' has length {len}"); // s still valid

    let mut msg = String::from("hi");
    append_excl(&mut msg); // mutable borrow
    println!("after mutate: {msg}");

    // Shared vs exclusive borrows can't overlap:
    let mut v = String::from("data");
    let r1 = &v;
    let r2 = &v; // multiple shared borrows are fine
    println!("{r1} {r2}");
    let r3 = &mut v; // ok now: r1/r2 no longer used after this point
    r3.push_str("!");
    println!("{r3}");
}

fn calc_len(text: &String) -> usize {
    text.len()
} // borrow ends; nothing dropped (we didn't own it)

fn append_excl(text: &mut String) {
    text.push('!');
}
