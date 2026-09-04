#![allow(unused)]

use std::fs::File;
use std::io::{ self, Read };
use std::error::Error;


// learn_09_01_box — Box<T>: single-owner heap allocation.
// Run: cargo run --bin learn_09_01_box
// Use Box for large data or recursive types (size unknown at compile time).

/*
 Smart Pointer:
 - pointer with metadata & additional capabilities


 Box:
 - allows data to be stored on the heap
 - useful for the data where the size is not known at compliet time
   - eg: Trat Objects, Recursive data structure

*/


// ex: Trat Objects
fn read_file(path: &str) -> Result<i32, Box<dyn std::error::Error>>{ 
     // dynamic return type 
     // here std::io::Error & std::num::ParseIntError both implements std::error::Error
 let mut file = File::open(path)?; // std::io::Error
 let mut data = String::new();
 file.read_to_string(&mut data)?; // std::io::Error
 let num: i32 = data.parse()?; // std::num::ParseIntError

 Ok(num)

}


// ex: Recursive data structure
#[derive(Debug)]
#[allow(dead_code)] // fields are read only via Debug; kept for the teaching example
enum List {
    // Cons(i32)  // compile time error...rust should know size at compile time
    Cons(i32, Box<List>), // recursive: Box gives it a known size (a pointer)
    Nil,
}
use List::{Cons, Nil};

// ex-2: Recursive data structure
#[derive(Debug)]

struct Tree {
    val: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>

}

fn main() {

    let path = "C:/work_spaces/rust/learn-rust/learn_09_smart_pointers/src/bin/number.txt";

    let s = read_file(path);

    println!("s:{:?}",s.unwrap());




    let i:i32 = 1;

    let b = Box::new(i); // now value 1 stored on heap

    let v = *b; // de-reference

    println!("v:{}",v);
    println!("b:{b}"); // // auto-deref


    // A recursive linked list, only possible via Box.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {list:?}");

    // ex-2
    let tree = Tree {
        val:1,
        left : Some(Box::new(Tree{
            val:2,
            left: None,
            right: Some(Box::new( Tree{
                val:3,
                left: None,
                right: None
            } ))
        })),
        right: Some(Box::new(Tree{
            val:4,
            left: None,
            right: None
        }))
    };
    
    println!("tree:{:?}", tree);

    println!("debug tree: {:#?}", tree);

    println!("tree.left.right.val : {} ", tree.left.unwrap().right.unwrap().val);


}
