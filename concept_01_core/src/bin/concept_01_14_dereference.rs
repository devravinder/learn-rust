
fn main(){
  let i = 12;
  let j = 13;


  let k = &i;// reference of 'i' is assigned to k
  let l = &j;

//   println!("add:{}", add(k,l)); error ... we can't pass reference

    println!("add:{}", add(*k, *l));


  let m = *k; // de-refeence  = remove reference & get the original
  let n = *l;

  let z = m + n;
 println!("z:{z}");

  let z = k + l; // addding reference works  // automatic de-reference for integers
  println!("z:{z}");

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}