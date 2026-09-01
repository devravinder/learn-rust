#![allow(unused)]
/*
 lifetimes tells rust compiler ...how long a value is valid
*/

// as the function is depending on two args....we should mention life time
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    // single 'a means all the variables have same life time
    if x.len() > y.len() {
         x
    } 
    else {
         y
    }
}

fn main(){

    // ===== 1st 
    let x = "🦀".to_string();
    let y = "🐍🐍".to_string();
    let z = longest_str(&x, &y);

    println!("z: {z}");

    //====   2nd

    let x = "🦀".to_string();

    {
        let y = "🐍🐍".to_string();

        let z = longest_str(&x, &y);
        println!("z: {z}");
    };

    //==== 3rd error
    /*

    let x = "🦀".to_string();

    let z = {
        let y = "🐍🐍".to_string();

        longest_str(&x, &y)
        // y is dropped here
    };
    // println!("z: {z}"); // error

     */

}