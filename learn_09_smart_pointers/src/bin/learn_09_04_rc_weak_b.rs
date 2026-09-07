#![allow(unused)]
use std::rc::{Rc, Weak};
use std::cell::RefCell;


/*
 Memory Leak
 RC<T> vs Weak<T>

 RC::strong_count() vs RC::weak_count()
*/

/*
Memory Leak:-
  - Allocated memory is not longer is accessable but not freed
  - variable is no longer accessbale but it still exists in memory


Issue with Rc ( or strong reference )
  - if we create cyclic references... this will create memory leak issue...as it can't release references( drop )
  
  - to solve this we use weak references ( see 'b' )


Weak<T>:-
  - it's a reference to the data that may be alive or de-allocated
  - increaments weak count
  - data can be dropped even if weak_count > 0

How to use Weak<T>:-
  - Rc::downgrade()
    - can't access the data behind the reference

  - Rc::upgrade()
    - upgrades weak to strong reference ( Weak<T> -> Strong<T> )
    - strong count increases
    - can access data behind the reference
   
*/

#[derive(Debug)]

struct Node {
  val: u32,
  neighbours: RefCell<Vec<Weak<Node>>>
}


fn main(){


  // basic usage 

  let x = "Rust".to_string();
  let r0 = Rc::new(x);

  let w1 : Weak<String> = Rc::downgrade(&r0);
  println!("----w1--------");
  println!("strong_count :{}", Rc::strong_count(&r0));
  println!("weak_count :{}", Rc::weak_count(&r0));
  println!("w1: {:#?}", w1);



  
  let w2 : Weak<String> = Rc::downgrade(&r0);
  println!("----w2--------");
  println!("strong_count :{}", Rc::strong_count(&r0));
  println!("weak_count :{}", Rc::weak_count(&r0));
  println!("w2: {:#?}", w2);


  let u1 = w1.upgrade();
  println!("----u1--------");
  println!("strong_count :{}", Rc::strong_count(&r0));
  println!("weak_count :{}", Rc::weak_count(&r0));
  println!("u1: {:#?}", u1); // if value exists Some else None

  // drop all strong references
  drop(u1);
  drop(r0);


  
  let u1 = w1.upgrade();
  println!("----u1--------");
  println!("u1: {:#?}", u1); // None



  println!("=========================");

  /*
   Reference Cycle

   node 0 -> node 1
   node 1 -> node 0

  */


  // 1st referenec to node0
  let node0 = Rc::new(Node {
    val: 0,
    neighbours: RefCell::new(vec![])
  });

  let node1 = Rc::new(Node {
    val: 1,
    neighbours: RefCell::new(vec![])
  });

  {
    let mut r0 = node0.neighbours.borrow_mut();
    r0.push(Rc::downgrade(&node1));
    
    
    
    let mut r1 = node1.neighbours.borrow_mut();
    // 2nd referenec to node0
    r1.push(Rc::downgrade(&node0));
  }

  // no infinite loop with weak reference
  println!("{:#?}", node0);



  // Memort leak
  println!("node0 strong_count : {}", Rc::strong_count(&node0));
  println!("node1 strong_count : {}", Rc::strong_count(&node1));

  std::mem::drop(node1); // if there is not strong reference...it'll drop

  println!("node0 strong_count : {}", Rc::strong_count(&node0)); // node1 exists or not...reference count remains the same




}