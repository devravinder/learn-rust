// learn_30_05_pattern_matching — match patterns in depth.
// Run: cargo run --bin learn_30_05_pattern_matching
// Ranges, bindings, guards, tuples, `_` wildcard, `|` or-patterns.
fn main() {
    // Ranges and wildcard.
    for n in [0, 5, 50, 200] {
        let label = match n {
            0 => "zero",
            1..=9 => "single digit",
            10..=99 => "double digit",
            _ => "big", // required: match must be exhaustive
        };
        println!("{n} -> {label}");
    }

    // Or-patterns and guards (extra condition with `if`).
    for n in [2, 3, 4, 9] {
        let kind = match n {
            2 | 3 | 5 | 7 => "prime-ish",
            x if x % 2 == 0 => "even",
            _ => "other",
        };
        println!("{n} -> {kind}");
    }

    // Destructuring a tuple with binding.
    let point = (0, 7);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("on y-axis at {y}"),
        (x, 0) => println!("on x-axis at {x}"),
        (x, y) => println!("at ({x}, {y})"),
    }
}
