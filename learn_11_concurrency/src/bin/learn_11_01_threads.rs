// learn_11_01_threads — OS threads with std::thread.
// Run: cargo run --bin learn_11_01_threads
// spawn returns a JoinHandle; join() waits. `move` gives the thread its own data.

/*
 Concurrency:-
  - Thread
  - Channel
  - Async / await programming
*/

use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;


fn main() {
    let mut handles = vec![];

    for id in 0..4 {
        let h: JoinHandle<String> = thread::spawn(move || {

            thread::sleep(Duration::from_millis(100));
            // each thread owns its captured `id`
            println!("{id}");
            format!("thread {id} done")
        });
        handles.push(h);
    }

    // join() blocks until the thread finishes and yields its return value.
    for h in handles {
        let msg = h.join().unwrap();
        println!("{msg}");
    }


    println!("--------");

    let h = thread::spawn(|| {
        return  1u32;
    });

    match h.join() {
        Ok(val)=> println!("val : {val}"),
        Err(err)=>println!("Error {:?}",err)
    }

}
