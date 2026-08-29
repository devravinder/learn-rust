// learn_12_03_async_channels — async message passing with tokio::mpsc.
// Run: cargo run --bin learn_12_03_async_channels
// Like std mpsc but await-able and used between async tasks.
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(16); // bounded channel, capacity 16

    // Producer task.
    tokio::spawn(async move {
        for i in 1..=5 {
            println!("in spawn {i}");
            tx.send(i * 10).await.unwrap(); // await if the buffer is full
        }
        // tx dropped here -> receiver loop ends
    });

    // Consumer: recv().await yields None once all senders are dropped.
    let mut total = 0;
    while let Some(v) = rx.recv().await {
        println!("received {v}");
        total += v;
    }
    println!("total = {total}");
}
