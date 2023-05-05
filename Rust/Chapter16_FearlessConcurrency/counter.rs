// we want to spawn n threads and have them increment a shared counter

use std::sync::{Arc, Mutex};
use std::thread;

fn main(){
    // to use a mutex accross threads we must use a Arc<T> or Atomic Reference Count
    // works similarly to Rc<T> reference count, but is safe for concurrent applications
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Counter value {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result {}", *counter.lock().unwrap());
}
