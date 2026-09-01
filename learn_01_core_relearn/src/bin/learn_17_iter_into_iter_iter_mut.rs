#![allow(unused)]
/*
Iter, Into Iter, Iter Mut
*/
fn main(){

    let values = vec![1,2,3,4];

    /*
    // Vec<T>
     iter(): borrows and returns a iterator that returns &T

     into_iter(): takes ownership and returns a iterator that may return T, &T or &mut T

     iter_mut(): returns &mut T
     */

 

     // iter()
      let values = vec![1,2,3,4];

      for v in values.iter() { println!("iter-1 v:{}", v);}
      for v in values.iter() { println!("iter-2 v:{}", v);}


    // into_iter
    for v in values{ println!("into_iter-1:{}", v);} 
    // for v in values.into_iter(){ println!("into_iter-1:{}", v);} // the both are same ( values, values.into_iter )
    //  only one time...after the move... we can't event print

    // println!("into_iter v: {:?}",values);

    // error second time -> `into_iter` takes ownership of the receiver `self`, which moves `values`
    // for v in values{ println!("into_iter-2:{}", v);} // value used here after move


    // iter_mut()

   let mut values = vec![1,2,3,4];

    for v in values.iter_mut() { 
        *v += 1;
        println!("iter_mut-1 v:{}", v);
    }

    println!("iter_mut v: {:?}",values);

    for v in values.iter_mut() { 
        *v += 1;
        println!("iter_mut-2 v:{}", v);
    }
    
    println!("iter_mut v: {:?}",values);




}