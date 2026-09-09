#![allow(unused)]
use std::rc::Rc;
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

   
*/

#[derive(Debug)]

struct Node {
  val: u32,
  neighbours: RefCell<Vec<Rc<Node>>>
}


fn main(){


  // basic usage

  let x = "Rust".to_string();
  let r0 = Rc::new(x);

  println!("Strong_count : {}", Rc::strong_count(&r0));

  {
    let r1 = Rc::clone(&r0);
      println!("Strong_count : {}", Rc::strong_count(&r0));

  }
  println!("Strong_count : {}", Rc::strong_count(&r0));

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
    r0.push(Rc::clone(&node1));
    
    
    
    let mut r1 = node1.neighbours.borrow_mut();
    // 2nd referenec to node0
    r1.push(Rc::clone(&node0));
  }

  // infinite loop
  // println!("{:#?}", node0);



  // Memort leak
  println!("node0 strong_count : {}", Rc::strong_count(&node0));
  println!("node1 strong_count : {}", Rc::strong_count(&node1));

  std::mem::drop(node1);// it won't drop ...as there is a strong reference ( as strong_count != 0 ). But we no longer have access to node1

  println!("node0 strong_count : {}", Rc::strong_count(&node0)); // if node1 is dropped node0 strong count should drop to 1... but not




}