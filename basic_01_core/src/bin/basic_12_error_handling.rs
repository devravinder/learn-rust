use std::io;
use std::cmp::Ordering;
use rand::prelude::*;
fn main(){
    let secret_number = rand::rng().random_range(1..=10); // range 1 to 10 // including 10
                     // gen_range(1..10) // range 1 to 10  // excluding 10



     loop {
      println!(" secret_number = {secret_number}");
      println!("Guess the number 1 to 10");
      let mut guess = String::new(); // use everytime a new buffer...else it may preserve previous values

      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

      
      /*
       guess.trim().parse() retruns a Result Enum, it has OK, Err values
       that we are passing to match statement
      */

      let guess : u128 = match guess.trim().parse() {
        Ok(num)=> num,
        Err(e)=>{
          println!("parsing error {}, try again", e);
          continue;
        }
      };


      match guess.cmp(&secret_number) { // reference
          Ordering::Less => println!("Too small!"), // -1
          Ordering::Greater => println!("Too big!"), // 1
          Ordering::Equal => {
            println!("You win!");
            break;
          }
      }
     }
}