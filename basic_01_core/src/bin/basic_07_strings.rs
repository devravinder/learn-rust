use std::io;

fn main(){
    println!("=========strings=========");

    let name = String::from("ravinder");
    println!("name: {name}");



    let mut full_name = String::new();



    println!("Please modify your name");

    // io::stdin().read_line(&mut  full_name); // mutable reference

    io::stdin().read_line(&mut  full_name)
    .expect("Error reading input");

    println!("Your new name: {full_name}")
}