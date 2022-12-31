use std::time::Duration; 
use std::thread;

fn main() {
    println!("More examples of closures...");

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(2);

    // closure immutable borrowing
    let list1 = vec![1,2,3];
    let only_borrows = || println!("The list: {:?}", list1);
    println!("Before closure {:?}", list1);
    only_borrows();
    println!("After closure {:?}", list1);

    // closure mutable borrowing
    let mut list2 = vec![1,2,3];
    println!("Before closure defined {:?}", list2);
    let mut mut_borrows = || list2.push(4);
    mut_borrows();
    println!("After closure {:?}", list2);

    // moving ownership to a clousre
    // useful for moving ownership to a new thread
    let list3 = vec![1,2,3];
    println!("Before defining closure: {:?}", list3);
    thread::spawn(move || println!("From Thread: {:?}, PID: {:?}", list3, thread::current().id()))
        .join()
        .unwrap();
}