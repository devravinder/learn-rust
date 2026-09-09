// concept_11_02_rc_refcell — Rc<T> shared ownership + RefCell<T> interior mutability.
// Run: cargo run --bin concept_11_02_rc_refcell
// Rc = multiple owners (single-thread, reference counted).
// RefCell = mutate through a shared ref, borrow-checked at RUNTIME.
use std::cell::RefCell;
use std::rc::Rc;

/*
Rc - Reference Count
   - allows muliple references ( owners )
     - use to share ownership for read only purpose
   - keeps track of no of references to the values wrapped in RC
   - RC count increases by 1 when RC is cloned
   - RC count decreases by 1 when RC is dropped
   - Cloning an RC never performs a deep copy
   - Single threaded use RC ( for Multi threaded use ARC - Atomic Reference Count )

watch: https://www.youtube.com/watch?v=_7O1qYMdGGA&list=PLO5VPQH6OWdXR8NlZt0jRbC39W_IyzS-v&index=55

 Target:   
 3 -> 2 -> 1 -> Nil
      ↓  
 4 <- ↓

 two references at 2 ( one is 3 & another is 4 )

*/

// without RC
#[derive(Debug)]
#[allow(dead_code)]
enum BoxList<'a> {
    BoxCons(i32, Box<&'a BoxList<'a >>),
    BoxNil,
}

use crate::BoxList::{BoxCons, BoxNil};


// with Rc
#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
// use List::{Cons, Nil}; // same



fn main() {

    let nil = BoxNil;
    let one = BoxCons(1, Box::new(&nil));
    let two = BoxCons(2, Box::new(&one));

    /* // muliple ownser - not possible with simple Box  ( without lifetime)

    let a = Cons(3, BoxCons::new(two));  // box taken ownership of list...so we can't use next time
    let b = Cons(4, BoxCons::new(two));  // error : value used here after move

    */

    // Solution - 1  ( without RC ) ( with lifetime )

    let a = BoxCons(3, Box::new(&two)); 
    let b = BoxCons(4, Box::new(&two));


    println!("a: {:?}",a);
    println!("b: {:?}",b);



    // Solution - 2

    println!("Solution-2 with Rc");
    
    let list = Rc::new(Cons(2, Rc::new(Cons(1,Rc::new(Nil)))));

    println!("Ref Countt {}", Rc::strong_count(&list));

    
    let a = Cons(3, Rc::clone(&list)); 
    println!("Ref Countt {}", Rc::strong_count(&list));
    {
        
        let b = Cons(4, Rc::clone(&list));
        println!("Ref Countt {}", Rc::strong_count(&list));
        println!("b: {:?}",b);
    }
    println!("Ref Countt {}", Rc::strong_count(&list));



    
    println!("a: {:?}",a);

    // Good excersize = print the list by dereferencing the RCs   // ******

    let mut cur : &List = &a;

    // v is ref to value  ( v = &i32 )
    // tail is ref to Rc<List> ( tail = &Rc )

    while let Cons(v, tail) = cur {
        
        print!("{v} -> ");

        // cur = &**tail  // two references so ...two times de-reference
        cur = tail // both are same
          /*
           currently (actually) 
            cur = &List
            tail = &Rc<List>
              // Deref Coercion = De-referencing to the suitbale type by unwrapping
              // here Rust converts &Rc<List> to &List
            
          */

    }
    println!("Nil");



    println!("-------------------");



    // old examples
    // Rc: clone bumps the count; value dropped when count hits 0.
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("value='{a}' count={}", Rc::strong_count(&a)); // 2
    drop(b);
    println!("after drop, count={}", Rc::strong_count(&a)); // 1

    // Rc<RefCell<T>>: shared AND mutable (common pattern for graphs/shared state).
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone = Rc::clone(&shared);
    clone.borrow_mut().push(4); // mutate through a shared handle
    println!("shared vec = {:?}", shared.borrow());
}
