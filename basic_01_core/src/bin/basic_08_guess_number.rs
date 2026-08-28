use std::io;
use rand::prelude::*;
fn main(){
      // install rand = cargo add rand

    println!("=========guess_number===========");


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


    if guess == secret_number {
      println!(" Guess is correct ");
    }else {
        println!("Wrong....the secret_number is : {secret_number}");
    }

}