#![allow(unused)]

/*
 Iterator adopters for map, filter, collect, zip & fold
*/

use std::collections::{HashMap, HashSet};

fn main(){

    let values = vec![1,2,3,4];
    let data : Vec<i32> = values.iter().map(|x| x*2).collect();
    println!("after map: {:?}", data);

    let data : HashSet<i32> = values.iter().map(|x| x*2).collect();
    println!("after collect: {:?}", data);

    let values = vec![1,2,3,4];
    // let data : Vec<i32> = values.iter().filter(|x| **x <= 2).copied().collect();
    let data : Vec<i32> = values.iter().filter(|&&x| x <= 2).copied().collect(); // both are same

    println!("after filter: {:?}", data);

    //=== Zip
    // useful to iterate over two iterators ( vectors )
    let keys = vec!["a", "b", "c", "d"];
    let values = vec![1,2,3,4];

    // iterates to minimum length iterator...skips extra ( here "d" is skipped )
    let zipped: Vec<(&str, i32)> = keys.into_iter().zip(values.into_iter()).collect();
    println!("zippe Vec: {:?}", zipped);

    //---- 
    let keys = vec!["a", "b", "c", "d"];
    let values = vec![1,2,3,4];

    let zipped: HashMap<&str, i32> = keys.into_iter().zip(values.into_iter()).collect(); // rust auto converts

    println!("zippe HashMap: {:?}", zipped); 


    // fold = is like reduce
    let values = vec![1,2,3,4];

    let sum = values.into_iter().fold(0, |acc, x| acc + x);

    println!("Sum: {sum}");




}