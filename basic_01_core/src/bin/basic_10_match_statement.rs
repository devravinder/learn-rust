use std::io;
use std::cmp::Ordering;
use rand::prelude::*;
fn main(){
  let secret_number = rand::rng().random_range(1..=10); // range 1 to 10 // including 10
                     // gen_range(1..10) // range 1 to 10  // excluding 10


    println!(" secret_number = {secret_number}");


    println!("Guess the number 1 to 10");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        
        // shadowing guess variable ( to use as integer type )
     let guess : u128 = guess.trim().parse()
     .expect("Parse error, invalid number input");   


    let message = match guess.cmp(&secret_number) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => {
            // Multiple statements
            println!("Super!");
            // last value  is the return value ( it's value )
            "You win!"
        }
    };
    println!("{message}")
}