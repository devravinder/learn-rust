// learn_01_03_data_types — scalar & compound types.
// Run: cargo run --bin learn_01_03_data_types
// Unlike JS's single `number`, you pick size/signedness (like Java).
fn main() {
    // Scalars: integers (iN/uN), floats (f32/f64), bool, char.
    let unsigned8: u8 = 255; // 0..=255
    let unsigned16: u16 = 1245;
    let unsigned32: u32 = 11233434;
    let unsigned64: u64 = 123455;
    let unsigned128: u128 = 123455;
    let unsigned: usize = 12343556; // size depends on platform


    let signed8: i8 = -42;
    let signed16: i16 = -142;
    let signed32: i32 = -242;
    let signed64: i64 = 9_000_000_000; // underscores are separators
    let signed128: i128 = 79_9_000_000_000; // underscores are separators
    let signed: isize = 9727727;// size depends on platform

    let float32: f32 = 3.14159;
    let float64: f64 = 3.14159;

    let is_present: bool = true; 
    let is_lazy: bool = false;

    let letter: char = 'R'; // 4-byte Unicode scalar, not a byte
    let crab: char = '🦀';

    println!("{unsigned8}, {unsigned16}, {unsigned32}, {unsigned64}, {unsigned128}, {unsigned}");
    println!("{signed8}, {signed16}, {signed32}, {signed64}, {signed128}, {signed}");
    println!("{float32}, {float64}");

    println!("{is_present},{is_lazy}");

    println!("{signed32} {unsigned8} {signed64} {float64} {letter} {crab}");

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
