use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hello number {} from spawned thread", i+1);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("Hello number {} from main thread", i+1);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // moving ownership of v to the thread with `move`
    let v = vec![1,2,3];
    let t = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    t.join().unwrap();
}