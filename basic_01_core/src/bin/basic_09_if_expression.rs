use std::io;
use rand::prelude::*;
fn main(){
     println!("=========if_expression===========");


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


     let mut message =  if guess == secret_number {
       String::from("Guess is correct")
      //  no ';' at the end....as it is in the middle of express...not end of expression
    }else if guess < secret_number{
       String::from("Guess is low")
    }else {
        String::from("Guess is high")
    };

    println!("{message}");
}