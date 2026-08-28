// learn_04_02_hashmap — HashMap<K, V> key/value store.
// Run: cargo run --bin learn_04_02_hashmap
// Like Java HashMap / JS Map. get() returns Option; entry() upserts.
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("alice".to_string(), 10);
    scores.insert("bob".to_string(), 7);

    println!("alice = {:?}", scores.get("alice")); // Option<&i32>
    println!("missing = {:?}", scores.get("carol"));

    // entry(): insert-if-absent, then modify — great for counting.
    *scores.entry("alice".to_string()).or_insert(0) += 5;
    *scores.entry("carol".to_string()).or_insert(0) += 1;

    for (name, score) in &scores {
        println!("{name}: {score}");
    }

    // Word frequency count.
    let text = "the cat the dog the bird";
    let mut freq: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    println!("freq = {freq:?}");
}
