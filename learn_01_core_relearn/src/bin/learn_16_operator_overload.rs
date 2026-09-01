#![allow(unused)]
use std::ops::Add;

/*
 Add trait: https://doc.rust-lang.org/std/ops/trait.Add.html

 pub trait Add<Rhs = Self> {
    type Output;
    // Here, Output is an associated type defined inside the trait.

    // Required method
    fn add(self, rhs: Rhs) -> Self::Output;
       // The Output type associated with this particular implementation of Self.
}

Operator Overload:-

*/


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self; // Self means the current type, here -> Point 
    // type Output = Point; // both are same

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// addition based on Generic
#[derive(Debug, Copy, Clone, PartialEq)]
struct Location<T> {
    x:T,
    y: T
}

impl<T> Add for Location<T> 
where T: Add<Output = T> // the T should implement Add trait & also the retun same type output
{
     type Output = Self;
    // type Output = Location<T>; // both are same

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


fn main(){

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2; // operator(+) is overloaded to support Point struct
    println!("p3:{:?}", p3);


    let l1  = Location { x: 1.0, y: 2.0 };
    let l2 = Location { x: 3.0, y: 4.0 };
    let l3  = l1 + l2; // operator(+) is overloaded to support Location struct
    println!("l3:{:?}", l3);

}