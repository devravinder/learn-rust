// learn_09_03_arc — Arc<T>: atomic (thread-safe) reference counting.
// Run: cargo run --bin learn_09_03_arc
// Arc = Rc for multiple THREADS. Pair with Mutex to share mutable state safely.
use std::sync::{ Arc, Mutex };
use std::thread;

fn main() {
    // Rc<RefCell>
    // Arc<Mutex>

    let m: Mutex<i32> = Mutex::new(0);
    let counter = Arc::new(m);

    let c1 = Arc::clone(&counter);
    let c2 = Arc::clone(&counter);


    let h1 = thread::spawn( move || {
        let mut v = c1.lock().unwrap();
        *v += 1;
    });


    let h2 = thread::spawn( move || {
        let mut v = c2.lock().unwrap();
        *v += 1;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("arc : {:?}", counter);
    println!("arc : {:?}", *counter.lock().unwrap());


    println!("==========example-2========");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter); // each thread gets its own handle
        handles.push(
            thread::spawn(move || {
                let mut n = c.lock().unwrap(); // lock -> exclusive access
                *n += 1;
            })
        );
    }
    for h in handles {
        h.join().unwrap();
    }

    println!("final count = {}", *counter.lock().unwrap()); // 5
}
