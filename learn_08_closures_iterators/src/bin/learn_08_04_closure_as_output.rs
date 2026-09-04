#![allow(unused)]
/*
 closure as function output

 Fn - captures variable by &T
 FnMut - captures variable by &mut T
 FnOnce - captured variable by T

*/

// Fn - captures variable by &T
fn f_fn()-> impl Fn(i32)->i32{
    let v = 1;

    move |x| x + v // move the ownership of closure
}
//  of v with 'move'...else after 1st call it'll be dropped...we can't call 2nd time....error


fn f_fn_string()-> impl Fn()->String{
    let s = "hello".to_string();

    move || {
        println!("s :{}", s);
        // s // cannot move out of `s`, a captured variable in an `Fn` closure // `s` has type `String`, which does not implement the `Copy` trait
        s.clone()
    }
}

// FnMut - captures variable by &mut T
fn f_fn_mut()-> impl FnMut(){

    let mut s = "rust".to_string();

    move || {
        s += "-lang";
        println!("s :{}", s);
    }
      
}


//  FnOnce - captured variable by T
fn f_fn_once()-> impl FnOnce() -> String{

    let s = "world".to_string();

    move || {
        println!("fn_once :{}", s);
        s
    }
      
}

fn main(){

    let f = f_fn();


    println!("fn(1) :{}", f(1));
    println!("fn(1) :{}", f(1));

    let f = f_fn_string();

    println!("f_fn_string, {}", f());
    println!("f_fn_string, {}", f());

    let mut f = f_fn_mut();

    f();
    f();


    let f = f_fn_once();


    let s = f();
    // f(); // error ...only once
    println!("s -- {}", s);




}