// learn_02_03_slices — a slice is a borrowed VIEW into a contiguous sequence.
// Run: cargo run --bin learn_02_03_slices
// &str is a string slice; &[T] is an array/vec slice. No copying, just a window.
// slice = reference to continuos memory
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // string slice (start..end, end exclusive)
    let world = &s[6..]; // to the end
    println!("{hello} | {world}");

    // first_word returns a slice into the original string.
    println!("first word: {}", first_word(&s));

    // Array slices work the same way.
    let nums: [i32; 5] = [10, 20, 30, 40, 50];
    let middle: &[i32] = &nums[1..4]; // [20, 30, 40]
    println!("slice sum = {}", middle.iter().sum::<i32>());
}

// &str accepts both String (via deref) and string literals.
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i], // range index ( exulded i )
        None => s,
    }
}
