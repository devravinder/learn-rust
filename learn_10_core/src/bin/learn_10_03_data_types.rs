// learn_10_03_data_types — scalar & compound types.
// Run: cargo run --bin learn_10_03_data_types
// Unlike JS's single `number`, you pick size/signedness (like Java).
fn main() {
    // Scalars: integers (iN/uN), floats (f32/f64), bool, char.
    let signed: i32 = -42;
    let unsigned: u8 = 255; // 0..=255
    let big: i64 = 9_000_000_000; // underscores are separators
    let pi: f64 = 3.14159;
    let letter: char = 'R'; // 4-byte Unicode scalar, not a byte
    let crab: char = '🦀';
    println!("{signed} {unsigned} {big} {pi} {letter} {crab}");

    // Tuple: fixed-length, mixed types.
    let person = ("Ravinder", 30, true);
    let (name, age, _active) = person; // destructure
    println!("{name} {age}, index .0={}", person.0);

    // Array: fixed-length, same type, on the stack.
    let data: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0u8; 4]; // [0,0,0,0]
    println!("{data:?} len={} {zeros:?}", data.len());
    // Growable lists = Vec<T> (see learn_40).

    // No implicit numeric conversion; cast with `as`.
    let a: u8 = 10;
    let b: i32 = a as i32 + 5;
    println!("cast: {b}");
}
