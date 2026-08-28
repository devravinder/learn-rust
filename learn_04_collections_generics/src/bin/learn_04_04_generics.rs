// learn_04_04_generics — generic functions & structs.
// Run: cargo run --bin learn_04_04_generics
// Like Java/TS generics <T>. Bounds (T: Trait) constrain what T must support.
use std::fmt::Display;

// Generic function with a trait bound: T must be comparable.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

// Generic struct.
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Display> Pair<T> {
    fn show(&self) {
        println!("({}, {})", self.first, self.second);
    }
}

fn main() {
    println!("largest int = {}", largest(&[3, 7, 2, 9, 4]));
    println!("largest char = {}", largest(&['a', 'z', 'm']));

    let p = Pair { first: 1, second: 2 };
    p.show();
    let q = Pair { first: "x", second: "y" };
    q.show();
}
