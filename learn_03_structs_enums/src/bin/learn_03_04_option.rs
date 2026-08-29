// learn_03_04_option — Option<T> replaces null.
// Run: cargo run --bin learn_03_04_option
// enum Option<T> { Some(T), None }. The compiler forces you to handle None.
fn main() {
    let some_num: Option<i32> = Some(5);
    let nothing: Option<i32> = None;

    // Handle both cases with match.
    match some_num {
        Some(n) => println!("got {n}"),
        None => println!("nothing"),
    }

    // if let: concise when you only care about one variant. // ***
    if let Some(n) = some_num {
        println!("if let: {n}");
    }

    // Common combinators (like optional chaining / defaults):
    println!("unwrap_or: {}", nothing.unwrap_or(0));
    println!("map: {:?}", some_num.map(|n| n * 10)); // |n| is closure syntax // lamda / arrow function

    // A function returning Option.
    println!("{:?}", first_even(&[1, 3, 4, 7]));
    println!("{:?}", first_even(&[1, 3, 5]));
}

fn first_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}
