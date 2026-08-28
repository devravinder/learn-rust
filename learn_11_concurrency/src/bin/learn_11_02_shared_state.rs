// learn_11_02_shared_state — share mutable state with Arc<Mutex<T>>.
// Run: cargo run --bin learn_11_02_shared_state
// Arc = shared ownership across threads; Mutex = one-at-a-time mutation.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = c.lock().unwrap(); // acquire lock; auto-unlocks at scope end
            *n += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    println!("counter = {}", *counter.lock().unwrap()); // 10
}
