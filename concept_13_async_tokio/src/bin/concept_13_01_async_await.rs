// concept_13_01_async_await — async/await with the Tokio runtime.
// Run: cargo run --bin concept_13_01_async_await
// Very familiar from JS: async fn returns a Future (like a Promise); .await drives
// it. #[tokio::main] sets up the runtime (like Node's event loop) around main.
use tokio::time::{Duration, sleep};

// An async fn returns a Future; nothing runs until it is .await-ed.
async fn fetch(label: &str, ms: u64) -> String {
    println!("label:{label}");
    sleep(Duration::from_millis(ms)).await; // non-blocking sleep
    format!("{label} done after {ms}ms")
}

#[tokio::main]
async fn main() {


    println!("----main---------");

    // async returns Future type

    // fetch("a", 100); // some times...main thread will exit first...before the async functions
    // fetch("b", 100);



    // Sequential: each await finishes before the next starts (~150ms total).
    let a = fetch("A", 100).await;
    let b = fetch("B", 50).await;
    println!("sequential: {a} | {b}");
}
