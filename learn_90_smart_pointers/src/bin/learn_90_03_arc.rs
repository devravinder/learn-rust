// learn_90_03_arc — Arc<T>: atomic (thread-safe) reference counting.
// Run: cargo run --bin learn_90_03_arc
// Arc = Rc for multiple THREADS. Pair with Mutex to share mutable state safely.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter); // each thread gets its own handle
        handles.push(thread::spawn(move || {
            let mut n = c.lock().unwrap(); // lock -> exclusive access
            *n += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    println!("final count = {}", *counter.lock().unwrap()); // 5
}
