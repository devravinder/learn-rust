#![allow(unused)]
use std::cell::RefCell;
use std::rc::Rc;

/*
 in Rust...in general, we can't mutate data ( mutables ) with immutable reference
  let mut s = "hello";
  let s1 = &s
  s1 += "hi" // error 

  But
  Inter Mutability ( is a feature / concept )
   - allows data mutation even though there immutable references to that data 

   - 'RefCell' 
      - enables Inter Mutability
      - runtime error when borrowing rules are broken
          - there should be only one mutable reference at any time....else error
      - Single threded use
      - Refcell are used in combination with RC to create a mutable data with shared ownership

 
  Target:   
 2 -> 1 -> Nil
      ↓              then change '1' to '9'
 3 <- ↓



*/


#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Rc<RefCell<List>>),
    Nil,
}

use List::{Cons, Nil};
// use List::{Cons, Nil}; // same

fn main(){

   // Basic

   let mut  s = "hello".to_string();
   let s1 = &s;
   // s1 += "world";
   println!("s1:{}",s1);


   let s = String::from("rust");
   let r: RefCell<String> = RefCell::new(s);

   {
     let mut r1 = r.borrow_mut();
     *r1 += "lang";
     println!("r: {:?}", r); // RefCell { value: <borrowed> } if r1 is not dropped


     /*

     let mut r1 = r.borrow_mut();
     *r1 += "lang";
     println!("r: {:?}", r); // RefCell { value: <borrowed> } if r1 is not dropped

      - The borrowing rules are broken
         - there should be only one mutable reference.... but we are adding one more

     - this will get compile...but runtime error
     
     */



   }

   /* same as above 
   let mut r1 = r.borrow_mut();
   *r1 += "lang";
   
    println!("r: {:?}", r); // RefCell { value: <borrowed> } if r1 is not dropped
    drop(r1);
   */
   println!("r: {:?}", r); // RefCell { value: "rustlang" } if  there is no mutable borrows ( if r1 is not dropped )





    //=================== Advance
   let mut a = Rc::new(RefCell::new(Cons(1,Rc::new(RefCell::new(Nil)))));
        // *a means take out the outer reference ...then Cons(1,Rc::new(Nil))

    let b = Cons(2, Rc::clone(&a)); 
        
    let c = Cons(3, Rc::clone(&a));

    if let Cons(v, _) = &mut *a.borrow_mut(){
        // *a means -> Cons(1,Rc::new(Nil))...then add a mutable reference to edit ... &mut *a
        *v = 9
    }

    // value will get updated ( form 1 to 9 ) in every link....bcz of same reference
    println!("a:{:?}",a);
    println!("b:{:?}",b);
    println!("c:{:?}",c);

}