// learn_01_10_match — exhaustive pattern matching.
// Run: cargo run --bin learn_01_10_match
// Like switch, but exhaustive and an expression (returns a value).

// also see: learn_03_05_pattern_matching

use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    println!("--------number | exact match or default---------");
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"), // like default
    }

    println!("------------string(&str)------");
    let command = "start";

    match command { // supports only &str
        "start" => println!("Starting..."),
        "stop" => println!("Stopping..."),
        "pause" => println!("Paused"),
        _ => println!("Unknown command"),
    }


    println!("------------String------");
    let command = String::from("stop");

    match command.as_str() { // convert to &str
        "start" => println!("Starting..."),
        "stop" => println!("Stopping..."),
        "pause" => println!("Paused"),
        _ => println!("Unknown command"),
    }

    println!("------number | union----------");

    let number = 2;

    match number {
        1 | 2 | 3 => println!("Small"),
        4 | 5 | 6 => println!("Medium"),
        _ => println!("Large"),
    }

    println!("-----------range------------");

    let age = 25;

    match age {
        0..=12 => println!("Child"),
        13..=17 => println!("Teenager"),
        18..=59 => println!("Adult"),
        60..=120 => println!("Senior"),
        _ => println!("Invalid age"),
    }

    println!("-------with if guard---------");
    let number = 10;

    match number {
        n if n < 0 => println!("Negative"),
        n if n == 0 => println!("Zero"),
        n if n > 0 => println!("Positive"),
        _ => println!("Unknown"),
    }

    println!("----------with Option ennum---------");
    let number: Option<i32> = Some(10);

    match number {
        Some(n) => println!("Number = {n}"),
        None => println!("No number"),
    }

    println!("------------with result enum --------");

    let result: Result<i32, &str> = Ok(100);

    match result {
        Ok(value) => println!("Success: {value}"),
        Err(error) => println!("Error: {error}"),
    }

    println!("---------Multi statement--------");

    let number = 10;

    let message = match number {
        0 => {
            println!("Got zero");
            "Zero"
        }

        1..=10 => {
            println!("Between 1 and 10");
            "Small"
        }

        _ => {
            println!("Large number");
            "Large"
        }
    };

    println!("Message: {message}");


    println!("---------with structs---------");
    struct User {
        #[allow(unused)] // to disable unused variable warning
        name: String,
        age: u32,
    }

    let user = User {
        name: String::from("Ravinder"),
        age: 30,
    };

    match user {
        User { age: 30, .. } => println!("User is 30"), // .. means ignore the rest
        User { age, .. } => println!("User is {age}"),
    }


    let secret = rand::rng().random_range(1..=10);
    println!("secret = {secret}");

    println!("Guess 1 to 10:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("read failed");
    let guess: i32 = guess.trim().parse().expect("invalid number");

    println!("-----------with compare-------");

    let message = match guess.cmp(&secret) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    };
    println!("{message}");


}
