
/*
 Multi Thread Smart Pointers
  - Arc   -> Rc
  - Mutex -> RefCell
*/

use std::sync::{Mutex, MutexGuard};
use std::thread;


fn main(){
  println!("hello");

      let m: Mutex<i32> = Mutex::new(0);

      thread::scope(|scope|{
                // without move... we can barrow variables with scoped thread
                scope.spawn(|| {
                     let mut v : MutexGuard<'_, i32>= m.lock().unwrap(); // when we call lock ...this will block access to the mutex for all other threads
                    //  let mut v : MutexGuard<'_, i32>= m.lock().unwrap(); // when we lock 2nd time...the current thread will goto blocked state
                       
                       
                      //  panic!("thread filed");// lock acquired...but not released
                          // if the lock is not released...if other thred tries to acuire lock...it'll get error
                              // that's why when we call lock... we do unwarp() & will get Result ->  Error, or Some
                       
                        *v += 1;


                    // lock will be dropeed when it goes out of the scope ( here after this closure )

                });

                scope.spawn(|| {
                     let mut v : MutexGuard<'_, i32>= m.lock().unwrap();
                        *v += 1;
                });

      });


      println!("mutex : {:?}", m);



}