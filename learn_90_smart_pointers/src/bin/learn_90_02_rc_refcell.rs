// learn_90_02_rc_refcell — Rc<T> shared ownership + RefCell<T> interior mutability.
// Run: cargo run --bin learn_90_02_rc_refcell
// Rc = multiple owners (single-thread, reference counted).
// RefCell = mutate through a shared ref, borrow-checked at RUNTIME.
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Rc: clone bumps the count; value dropped when count hits 0.
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("value='{a}' count={}", Rc::strong_count(&a)); // 2
    drop(b);
    println!("after drop, count={}", Rc::strong_count(&a)); // 1

    // Rc<RefCell<T>>: shared AND mutable (common pattern for graphs/shared state).
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone = Rc::clone(&shared);
    clone.borrow_mut().push(4); // mutate through a shared handle
    println!("shared vec = {:?}", shared.borrow());
}
