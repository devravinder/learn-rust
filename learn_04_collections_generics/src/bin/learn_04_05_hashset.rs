use std::collections::HashSet;

fn main(){
    let mut scores: HashSet<i32> = HashSet::new(); // stores references

    let inserted = scores.insert(1);
    println!("inserted:{inserted}");

    let inserted = scores.insert(1);
    println!("inserted:{inserted}");

    let inserted = scores.insert(2);
    println!("inserted:{inserted}");

    let inserted = scores.insert(3);
    println!("inserted:{inserted}");

    println!("scores: {scores:?}"); // no order

    // println!("contains: {}",scores.contains(3)); // error -> pass reference 
    println!("contains: {}",scores.contains(&3)); // pass reference 
}