
/*
 Closure traits:-
  Fn, FnMut, FnOnce

    /*
     Closures can capture variables by
       - Borrow immutable reference &T   => Fn   ( capture by &)
          - immutable borrow from the environment
          - can be called more than oce
       - Borrow mutable reference &mut T => FnMut ( capture by &mut )
          - mutable borrow from the environment
          - can be called more than oce

       - Take ownership of value T       => FnOnce ( capture by value )
           - moves captured values into closure, if needed   ( takes ownership )
           - can be called at least once
          
    */

*/


fn f_fn(f: impl Fn()){
    f();
    f();
}

// fn f_fn<F: Fn()>(f: F){} // both are same // with generics


fn f_fn_mut<F: FnMut()>(mut f: F){
    f();
    f();
}

fn f_fn_once<F: FnOnce()>(f: F){
    f();
    // f(); only once
}


fn main(){

    // capture by &
    let s = "hello".to_string();
    let f = || println!("in fn :{}", s);
    f_fn(f);
    println!("after :{}", s); // borrowed...so works


    // capture by &mut
    let mut v = vec![0];
    let mut f = || v.push(1);

    f_fn_mut(f);
    println!("vec after: {:?}", v);


    
    // capture by value
    let v = String::from("Rust");  // ownership borrowed
    let greet = move || println!("fn once {}", v);

    f_fn_once(greet);

    // f_fn_once(greet); // error
    // println!(" name: {}", name); error // ownership is moved


     let v = 123; // ownership is copied
    let greet = move || println!("fn once {}", v);

    f_fn_once(greet);
    f_fn_once(greet); // ownership is copied...we can call multiple times



}