// learn_10_05_shadowing — re-declare a name with `let`.
// Run: cargo run --bin learn_10_05_shadowing
// Differs from `mut`: shadowing makes a NEW variable and can change type.
fn main() {
    let x = 5;
    let x = x + 1; // new x = 6

    {
        let x = x * 2; // inner scope only
        println!("inner x = {x}"); // 12
    }
    println!("x = {x}"); // 6

    // Type can change via shadowing (impossible with mut):
    let spaces = "   ";
    let spaces = spaces.len(); // &str -> usize
    println!("spaces = {spaces}");
}
