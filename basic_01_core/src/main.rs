fn main() {
    println!("============hello_world===============");
    hello_world();

    println!("===========variables================");
    variables();

    println!("==========debug_format==========");
    debug_format();
}

fn hello_world(){
  println!("Hello, world!"); // ! represents macros ( macros != functions )
}

fn variables(){
  let name = "Ravinder"; // by default variables are immutable
  println!("Name is {name}");

  let mut last_name = "Reddy";
  println!("Full name: {name} {last_name}");

  last_name = "Reddy Kothabad";

  println!("Full name: {name} {last_name}");
  println!("---------");

  println!("Full name: {} {}", name, last_name);

  println!("-------------");
  println!("Full name: {name} {} ", last_name.to_lowercase()); // with expression

}


fn debug_format(){
  let data = [1,2,3,4,5];
  println!("data: {data:?}")
}