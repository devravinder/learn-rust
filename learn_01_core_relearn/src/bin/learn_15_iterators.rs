#![allow(unused)]

/*
Associated Type:-
 - placeholder type inside trait definition
 - placeholde will be replaced by the implementation

Difference With Generic Trait:-
 - generic type: multiple implementations per type
 - associated type: 1 implementation per type


*/ 


// Generic Type
trait GenericIterator<T>{
    fn gen_next(&mut self) -> Option<T>;
}


struct ArrayIter<T> {
  array: [T;5],
  i: usize
}

impl GenericIterator<u32> for ArrayIter<u32> {
    fn gen_next(&mut self) -> Option<u32>{
        match self.array.get(self.i){
            Some(v)=>{
              self.i += 1;
              Some(*v)
            }
            _ => None
        }
    }
}

impl GenericIterator<bool> for ArrayIter<u32> {
    fn gen_next(&mut self) -> Option<bool>{
        Some(true)
        // actually we should return the next item... but we are returing...bool
        // ... but still it is compiling... not as expected
        // but associated type gives error -> check associated iterator ( as ecpected )
    }
}

// best way 
// Associated Iterator

// Associated Type
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for ArrayIter<u32> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
         match self.array.get(self.i){
            Some(v)=>{
              self.i += 1;
              Some(*v)
            }
            _ => None
        }
    }
}

/*
impl Iterator for ArrayIter<u32> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
         Some(true);
    }
}
*/

fn main(){


    let mut arr_iter:ArrayIter<u32> = ArrayIter {
        array:[1,2,3,4,5],
        i: 0
    };

    /* 

    // to test generic iterator... comment above the: impl Iterator for ArrayIter<u32>
    // then uncomment below code 
    // also comment the iterator based below code: arr_iter.next

    while let Some(v) = arr_iter.gen_next(){
        println!("{:?}", v);
    }
    */

    while let Some(v) = arr_iter.next(){
        println!("{:?}", v);
    }

}