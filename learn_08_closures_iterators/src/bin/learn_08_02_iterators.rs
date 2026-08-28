// learn_08_02_iterators — lazy iterator adapters.
// Run: cargo run --bin learn_08_02_iterators
// map/filter/collect etc. — like JS array methods, but LAZY until a consumer runs.
fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    // map + filter + collect (adapters are lazy; collect() drives them).
    let evens_squared: Vec<i32> = nums
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect();
    println!("evens squared: {evens_squared:?}");

    // Consumers that produce a single value.
    let sum: i32 = nums.iter().sum();
    let max = nums.iter().max();
    let count = nums.iter().filter(|&&n| n > 3).count();
    println!("sum={sum} max={max:?} count(>3)={count}");

    // fold (like reduce).
    let product = nums.iter().fold(1, |acc, &n| acc * n);
    println!("product = {product}");

    // enumerate + take + chain.
    for (i, n) in nums.iter().enumerate().take(3) {
        println!("#{i} = {n}");
    }

    // Build from a range, no intermediate Vec.
    let triples: Vec<i32> = (1..=5).map(|n| n * 3).collect();
    println!("triples: {triples:?}");
}
