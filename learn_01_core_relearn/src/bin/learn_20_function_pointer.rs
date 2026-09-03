#![allow(unused)]

fn add(x: u32, y: u32)-> u32{
    x + y
}

fn do_twice(f: fn(u32, u32)-> u32, x: u32, y: u32) -> u32{
   f(x, y) + f(x, y)
}

fn push(v: &mut Vec<u32>, x: u32){
    v.push(x)
}

fn do_push_twice(f: fn(&mut Vec<u32>, u32), x:&mut Vec<u32>, y: u32){
    f(x, y);
    f(x, y);
}

fn main(){
  let f: fn(u32, u32)-> u32 = add;
  println!(" add: {}", f(2,3));

  println!("add twice {}", do_twice(add, 2, 3));
  
  let mut v = vec![1,2,3];

  
  do_push_twice(push, &mut v, 2); // passing mutable reference


  println!("vec: {:?}", v);


}