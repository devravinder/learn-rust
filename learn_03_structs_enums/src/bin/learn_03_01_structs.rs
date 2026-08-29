// learn_03_01_structs — structs hold named fields (like a class's data).
// Run: cargo run --bin learn_03_01_structs
// Three forms: named-field, tuple struct, unit struct. #[derive(Debug)] for {:?}.
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

#[derive(Debug)]
struct Point(i32, i32); // tuple struct (fields by index)

fn main() {
    let mut u = User {
        name: String::from("Ravinder"),
        age: 30,
        active: true,
    };
    u.age += 1; // whole struct must be `mut` to change a field
    println!("{u:?}");
    println!("name={}, age={}, active={}", u.name, u.age, u.active);

    // Struct update syntax: copy remaining fields from another instance.
    let u2 = User {
        name: String::from("Kiro"),
        ..u // take age/active from u (moves non-Copy fields)
    };

        // this works
        println!("u:  {u:?}");
        println!("u2: {u2:?}");
        
        u.age += 1; 
        println!("u1: {u:?}");
        println!("u2: {u2:?}"); // u2 is not effected

        /*
        println!("u2: {u2:?}");
        println!("u1:{u1:?}"); // error ...after above line u1 is cleared
         */

    let p = Point(3, 4);
    println!("point x={} y={}", p.0, p.1);
}
