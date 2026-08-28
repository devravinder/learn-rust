// learn_03_02_methods — impl blocks add methods & associated functions.
// Run: cargo run --bin learn_03_02_methods
// &self = borrow, &mut self = mutating borrow, self = consumes. No `self` = static.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (no self) = constructor, like a static factory.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Method: borrows self (read-only).
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Mutating method.
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut r = Rectangle::new(3, 4); // call associated fn with ::
    println!("area = {}", r.area()); // call method with .
    r.scale(2);
    println!("scaled = {r:?}, area = {}", r.area());
}
