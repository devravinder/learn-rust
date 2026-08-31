#![allow(unused)]

fn my_print<T: std::fmt::Debug>(t:T){
    println!("{t:?}");// to use in print, it must implement Debug
}


//===
trait A {}
trait B {}
trait C {}

//--
impl A for u32 {}
impl B for u32 {}
impl C for u32 {}

//--
impl A for i32 {}

//---
fn k<T:A> (t:T){}
fn l<T:A+B> (t:T){}
// fn m<T:A+B, U:B+C> (t:T, u:U){} // same as below

fn m<T, U>(t:T, u:U)
where 
    T:A+B,
    U:B+C,
{}

//======= impl trait vs trait bound

// x & y can be different...but they should implment A
fn o(x: impl A, y: impl A){}

// x & y must be same type & they should implement A
fn p<T:A>(x: T, y: T){}

// x & y can be different...they should implement A
fn q<T:A, U:A>(x: T, y: U){}


fn main(){
  let t = (12, 12);
  my_print(t);

  let u: u32 = 32;
  let i: i32 = -32;
  let f: f32 = 32.0;

  //---
  k(u);
  k(i);
  // k(f);// compilation error: f32 is not implemented A

  //--
  l(u);
//   m(i);// compilation error: i32 is not implemeted B... it is implemented only A


//==
o(i,i);
o(i,u);

p(i,i);
// p(i,u);// error..difeerent data types

q(i,u);

}