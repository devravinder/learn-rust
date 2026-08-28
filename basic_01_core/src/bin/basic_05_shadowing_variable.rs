fn main(){

  println!("=================shadowing_variable=================");
    let x = 5;

    let x = x + 1;

    {
      // the second variable overshadows the first
        let x = x * 2; // the first variable is shadowed by the second

        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}