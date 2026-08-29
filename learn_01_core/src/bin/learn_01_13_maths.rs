fn main(){
    let x = 127_i64;
    let y = 10_i32;

    let z = x + y as i64;
    println!("{x}+{y}={z}");

    let i_max = i32::MAX;

    println!("i_max={i_max}");

}