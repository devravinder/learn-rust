// learn_11_03_channels — message passing with mpsc channels.
// Run: cargo run --bin learn_11_03_channels
// "Do not communicate by sharing memory; share memory by communicating."
// mpsc = multi-producer, single-consumer. Like a Go channel / an event queue.
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;


fn main() {
    let (tx, rx) : (Sender<String>, Receiver<String>) = mpsc::channel();

    //=== basic - blocking 

    tx.send("Hello-1".to_string()).unwrap();
    let res = rx.recv(); // main thread waits until message is received
    println!("res-1:{:?}", res);

    //
    // std::mem::drop(rx);
    // tx.send("Hello".to_string()).unwrap(); // this will give error...if there is no receiver


    let tx2 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx2.send("Hello-2".to_string()).unwrap();
    });
    
    
    let res = rx.recv(); // main thread waits until message is received
    println!("res-2:{:?}", res);

    // non-blocking


    println!("===========================");

    let tx3 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx3.send("Hello-3".to_string()).unwrap();
    });
    
    loop {
       match rx.try_recv(){
          Ok(res)=>{
            println!("res-3,{:?}", res);
            break;
          },
          // Err(err)=>println!("Error {:?}",err)
          Err(TryRecvError::Empty)=>println!("Error no message"),
          Err(TryRecvError::Disconnected)=>{
            println!("Error - Disconnected");
            break; // else it'll continue
          }
        }
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("========================");

    // Multiple producers: clone the sender.
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            tx.send(format!("msg from {id}")).unwrap();
        });
    }
    drop(tx); // drop the original so the receiver knows when all senders are gone
    println!("sender is dropped");

    // Receiver iterates until all senders are dropped.
    for received in rx {
        println!("got: {received}");
    }
}
