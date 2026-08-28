// learn_11_01_threads — OS threads with std::thread.
// Run: cargo run --bin learn_11_01_threads
// spawn returns a JoinHandle; join() waits. `move` gives the thread its own data.
use std::thread;

fn main() {
    let mut handles = vec![];

    for id in 0..4 {
        let h = thread::spawn(move || {
            // each thread owns its captured `id`
            format!("thread {id} done")
        });
        handles.push(h);
    }

    // join() blocks until the thread finishes and yields its return value.
    for h in handles {
        let msg = h.join().unwrap();
        println!("{msg}");
    }
}
