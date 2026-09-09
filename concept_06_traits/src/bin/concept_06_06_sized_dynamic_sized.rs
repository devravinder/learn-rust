#![allow(unused)]
/*

Sized vs ? Sized

Sized:
 - size known at compile time.
 - automatically implemented for primitive types
 - necessary for allocating size on stack

 ?Sized:
  - size may not known at compile time
  - eg: dynamically sized types, slices, traits

*/ 

fn f<T: Sized>(t:T){}
fn g<T: ?Sized>(t: &T){} // for dynamic size...we should pass reference ( &T )... not type


//===

trait A {}

impl A for u32{}

fn d(x: Box<dyn A>){} // dynamic trait

fn main(){

    println!(" Sized vs !Sized");
    
    // Sized
    // primitive types

    let u = 1u32;
    let i = 1i32;
    let b: bool = true;

    f(u);
    f(i);
    f(b);

    struct  S {
        u: u32,
        i: i32
    }
    
    let s: S = S{u:u, i:i};

    f(s);


    let arr: [i32; 4] = [1;4];

    f(arr);
    f(&arr); // both works....size of the array known at compile time


    // ?Sized

    let slice: &[i32] = &[1,2,4]; // while running program....we can slice it up & change the size

    g(slice);

    let str: &str = "hello";
    g(str);

    //---
    let v : Box<dyn A> = Box::new(1u32);

    g(&v);

}