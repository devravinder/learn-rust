#![allow(unused)]
// concept_09_01_closures — anonymous functions that capture their environment.
// Run: cargo run --bin concept_09_01_closures
// |args| body — like JS arrow functions. They can capture surrounding variables.

// Closure = anonymous function + capture variables in the environment

fn main() {
    let add = |a: i32, b: i32| a + b; // typed closure
    println!("add = {}", add(2, 3));


    println!("Type Inferred");

    let f = |x,y| x+y;
    let res = f(1,2);
    println!("res:{:?}",res);


        // let res = f(1.0,2.0); // once the type is inferred... you cannot change


     println!("--------------");

     let v = 1;

     let x = |x| x+v; // as the function is in main... it can acccess main variables

     let res = x(2);    

    println!("res:{:?}",res);


    println!("ownership -----------");

    /*
     Closures can capture variables by
       - Borrow immutable reference &T
       - Borrow mutable reference &mut T
       - Take ownership of value T
    */


    // Borrow immutable reference &T
    let factor = 10;
    let scale = |x: i32| x * factor; // borrows `factor`
    println!("scale(5) = {}", scale(5));

    println!("factor: {}", factor);// still accessable...bcz of borrow


    // Borrow mutable reference &T

    let mut s = "hello".to_string();
    let mut f = || s+="world";
    f();
    println!(" s: {}", s);


    // Take ownership of value T -> by move (takes ownership) — needed when returning/spawning.
    let name = String::from("Rust");
    let greet = move || { // FnOnce type cunction = once time callable function
        println!("name {}", name);
        name; // ownership of name is taken into closure 
              // also varaibles are dropped...so we can't use the function 2nd time

              // this is same for normal functions also... once the ownership is dropped...we can't re-use the variable
    };
     
     greet();

    //  greet(); // FnOnce can be called only once

    // Passing a closure to a function (FnMut for mutation).
    let mut count = 0;
    let mut bump = || count += 1;
    bump();
    bump();
    println!("count = {count}");

    // Returning a closure (boxed trait object) / accepting via generics.
    let doubler = make_multiplier(2);
    println!("doubler(21) = {}", doubler(21));


}

// Return an `impl Fn` closure.
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n // returing arrow function
}
