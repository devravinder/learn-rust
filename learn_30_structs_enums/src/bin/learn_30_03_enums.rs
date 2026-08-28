// learn_30_03_enums — enums are tagged unions (each variant can carry data).
// Run: cargo run --bin learn_30_03_enums
// Like TS discriminated unions / Java sealed classes. Match to handle each variant.
#[derive(Debug)]
enum Shape {
    Circle(f64),         // radius
    Rectangle(f64, f64), // width, height
    Triangle { base: f64, height: f64 }, // struct-like variant
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

fn main() {
    let shapes = [
        Shape::Circle(2.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle { base: 6.0, height: 2.0 },
    ];
    for s in &shapes {
        println!("{s:?} area = {:.2}", s.area());
    }
}
