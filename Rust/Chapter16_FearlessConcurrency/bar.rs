// a bar is hosting a football game
// it will allow 1 jets fan for each 2 patriots fans


use std::sync::mpsc::{channel, Sender, Receiver}; // we will use message passing
use std::thread; 

enum Message {
    EnterPat, 
    EnterJet,
    ACK,
    GoHome,
}

fn main() {
    let (tx0, rx0) = channel();
    let (num_patriots, num_jets) = (10, 5);
    
    // bouncer in charge of letting patriots/jets enter
    let bouncer_handle = thread::spawn(move || bouncer(rx0, 0));
    let mut fan_handles = vec![];

    for _ in 0..num_patriots {
        let tx = tx0.clone();
        let handle = thread::spawn(move || patriot(tx));
        fan_handles.push(handle);
    }
    
    for _ in 0..num_jets {
        let tx = tx0.clone();
        let handle = thread::spawn(move || jet(tx));
        fan_handles.push(handle);
    }
    
    // Wait for all fans to get in
    for handle in fan_handles {
        handle.join().unwrap();
    }
    // Once all fans are in, we can tell the bouncer to go home
    tx0.send((Message::GoHome, channel().0)).unwrap(); 
    bouncer_handle.join().unwrap();
}

fn patriot(s: Sender<(Message, Sender<Message>)>) {
    s.send((Message::EnterPat, channel().0)).unwrap();
}

fn jet(s: Sender<(Message, Sender<Message>)>) {
    let (tx, rx) = channel();
    s.send((Message::EnterJet, tx)).unwrap();
    match rx.recv(){ // block until ACK recieved 
        Ok(Message::ACK) => {},
        _ => {}
    }
}


fn bouncer( rx: Receiver<(Message, Sender<Message>)>, patriots: usize) {
    match rx.recv() {
        Ok((Message::EnterPat, _)) => {
            println!("Patriot In!");
            bouncer(rx, patriots + 1);
        }
        Ok((Message::EnterJet, tx)) if patriots > 1 => {
            tx.send(Message::ACK).unwrap();
            println!("Jet In, Patriots={}", patriots);
            bouncer(rx, patriots - 2);
        }
        Ok((Message::GoHome, _)) => {},
        _ => println!("This message means something broke")
    }
}

