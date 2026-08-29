// learn_01_11_loops — loop / while / for.
// Run: cargo run --bin learn_01_11_loops
// No C-style for; use ranges. `loop` can return a value via `break value`.
fn main() {
    // loop returning a value
    let mut counter = 0;
    let result = loop { // infinite loop ( until break )
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop returned {result}"); // 20

    // while
    let mut n = 3;
    while n > 0 {
        print!("{n} ");
        n -= 1;
    }
    println!();

    // for over range (1..=5 inclusive, 1..5 exclusive)
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // for with index
    let langs = ["Java", "TypeScript", "Rust"];
    for (i, lang) in langs.iter().enumerate() {
        println!("{i}: {lang}");
    }

    // labeled break
    'outer: for a in 0..3 {
        for b in 0..3 {
            if a + b == 3 {
                println!("break outer at a={a} b={b}");
                break 'outer;
            }
        }
    }
}
