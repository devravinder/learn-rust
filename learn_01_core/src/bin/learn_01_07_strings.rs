// learn_01_07_strings — String vs &str + stdin.
// Run: cargo run --bin learn_01_07_strings
// String = owned, growable (heap)
// &str = borrowed, read-only view. It is slice (Pointer). Known size at compile time
use std::io;
use std::any::type_name_of_val;

fn main() {
    let name = String::from("ravinder"); // &str -> String
    println!("name: {name}");

    let mut full_name = String::new(); // empty growable buffer
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3:{s3}");


    let mut name = String::from("Ravinder");

    name.push_str(" Reddy"); // we can modify String

    println!("{name}");


    let name = String::from("Ravinder");

    let user_name: &str = &name; // doesn't own...just borrows

    // user_name.push_str(" Reddy"); // ❌ gives error
    println!("user_name: {user_name}");


    let user_name = "Raamu"; // literals are &str ... borrows the reference // we can't modify
    println!("user_name: {user_name}");

    let mut user_name = "Raamu"; 
    println!("user_name before: {user_name}");

    user_name = "raaju"; // new literal created & reference is borrowed
    println!("user_name after: {user_name}");


    println!("---------Conversion---------");

    let name: String = String::from("Ravinder");
    let name : &str = name.as_str();

    println!("name: {name}, type: {}",type_name_of_val(&name));


    let name: String = name.to_string();

    println!("name: {name}, type: {}",type_name_of_val(&name));


    println!("---------derefer------------");

    let mut s = String::from("ravinder");

    let s1 = &mut s; // mutable barrow

    *s1 += "?"; // * is dereference

    println!("s1={s1}");

    
    println!("Type your name:");
    // read_line appends input (incl. newline); returns Result.
    io::stdin()
        .read_line(&mut full_name)
        .expect("Error reading input");

    println!("Your name: {}", full_name.trim());
}
