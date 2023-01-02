use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    println!("{}", *m.lock().unwrap()); // print value before
    {
        let mut num = m.lock().unwrap();
        *num = 6; 
        // mutex is unlocked once it comes out of scope
    }
    println!("{}", *m.lock().unwrap()); //print value after
}