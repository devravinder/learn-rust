// to convert one type(struct) to another
// do value-to-value conversions
// in Rust ( in docs ) Sized means ...known datatype ( known length )

/*
// https://doc.rust-lang.org/std/convert/trait.From.html
pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    // Required method
    fn into(self) -> T;
}

*/

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}


fn main(){
  println!("hello");

  let t = (32,43);
  let point_1 = Point::from(t);

  let point_2: Point = t.into();// explicity type declaration is needed...as we implemented `From`.. auto `into` will work

  println!("point_1: {point_1:?}");
  println!("point_2: {point_2:?}");



}