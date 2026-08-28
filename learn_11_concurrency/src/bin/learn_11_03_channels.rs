// learn_11_03_channels — message passing with mpsc channels.
// Run: cargo run --bin learn_11_03_channels
// "Do not communicate by sharing memory; share memory by communicating."
// mpsc = multi-producer, single-consumer. Like a Go channel / an event queue.
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Multiple producers: clone the sender.
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("msg from {id}")).unwrap();
        });
    }
    drop(tx); // drop the original so the receiver knows when all senders are gone

    // Receiver iterates until all senders are dropped.
    for received in rx {
        println!("got: {received}");
    }
}
