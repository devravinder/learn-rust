// learn_12_02_tasks — run futures concurrently with join! and spawn.
// Run: cargo run --bin learn_12_02_tasks
// join! awaits multiple futures at once (like Promise.all). spawn runs a task in
// the background on the runtime's thread pool.
use tokio::time::{Duration, sleep};

async fn work(label: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{label}({ms}ms)")
}

#[tokio::main]
async fn main() {
    // Concurrent: both run at the same time (~100ms total, not 150ms).
    let (a, b) = tokio::join!(work("A", 100), work("B", 50));
    println!("join!: {a} + {b}");

    // spawn returns a JoinHandle; await it for the result (like Promise.all over spawns).
    let mut handles = vec![];
    for i in 0..3 {
        handles.push(tokio::spawn(async move { work(&format!("task{i}"), 30).await }));
    }
    for h in handles {
        println!("spawned: {}", h.await.unwrap());
    }
}
