// learn_03_03_enums — enums are tagged unions (each variant can carry data).
// Run: cargo run --bin learn_03_03_enums
// Like TS discriminated unions / Java sealed classes. Match to handle each variant.


#[derive(Debug)]
enum TrfficLight {
  Red,
  Yellow,
  Green
}

#[derive(Debug)]
enum Shape {
    Circle(f64),         // radius
    Rectangle(f64, f64), // width, height
    Triangle { base: f64, height: f64 }, // struct-like variant // ***
    Multi (f64, f64, f64, f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
            _ => 0.0 // like default
        }
    }
}

fn main() {


    let light = TrfficLight::Red;

    match  light {
        TrfficLight::Red => println!("Stop"),
        TrfficLight::Yellow => println!("Ready"),
        TrfficLight::Green => println!("Go")
    }
    println!("{:?}, {:?}", TrfficLight::Green, TrfficLight::Yellow);

    


    let shapes = [
        Shape::Circle(2.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle { base: 6.0, height: 2.0 },
        Shape::Multi(1.0, 2.0, 3.0, 4.0)
    ];
    for s in &shapes {
        println!("{s:?} area = {:.2}", s.area()); // {:.2} // two decimals
    }
}
