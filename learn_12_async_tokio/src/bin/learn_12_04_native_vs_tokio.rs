
/*
 Native Threads vs Tokio Runtime
 Threads vs Async

 - Native Threads
   - good for CPU bound work
   - limited by memory & OS thread limit

- Async
  - less memory consumption
  - no limit on number of threads
     - this spawns virtula threads
  - good for IO bound computations


*/
use tokio::time::{Duration, sleep};



#[tokio::main]
async fn main(){

  /* 
  // native threads
  // this gives error after some time

  let mut handles = vec![];

  for i in 0..1000000 {
    let handle = std::thread::spawn(move || {
       std::thread::sleep(std::time::Duration::from_millis(10));
       println!("native thread :{i}");
    });

    handles.push(handle);
  }


  for h in handles {
    h.join();
  }

  */

  // Async 
  // works without any error

  let mut handles = vec![];

  for i in 0..1000000 {

    let fut = async move {
       sleep(Duration::from_millis(10)).await;
       println!("async thread :{i}");
    };

    let task = tokio::spawn(fut);
    handles.push(task);
  }


  for h in handles {
    h.await.unwrap();
  }


}