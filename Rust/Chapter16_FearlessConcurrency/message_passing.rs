use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main(){
    let (tx, rx) = mpsc::channel(); // create a channel which returns a transmitter (tx) and reciever (rx)
    thread::spawn( move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let recieved = rx.recv().unwrap(); // .recv() blocks until recieved
    println!("Got: {recieved}");

    // sending multiple values
    let (tx2, rx2) = mpsc::channel();

    let tx2_2 = tx2.clone(); // create a clone of second transmitter
                             // so that we can have multiple senders and one reciever
    thread::spawn(move || { // P 
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || { // Q
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx2_2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx2 {
        println!("Also Got: {recieved}");
    }
}