// spawn 2 threads which increment a counter
// stop when the counter reaches N

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let n: i32 = 20;
    let counter = Arc::new(Mutex::new(0)); 
    let mut handles = vec![];
    
    // spwan 2 threads
    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            loop {
                let mut num = counter.lock().unwrap();
                if *num < n { 
                    *num += 1;
                    println!("Thread: {:?}, incremented counter", thread::current().id());
                }
                else {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    // Join threads
    for handle in handles {
        handle.join().unwrap();
        println!("Thread joined...");
    }
    println!("Result {}", *counter.lock().unwrap());
}
