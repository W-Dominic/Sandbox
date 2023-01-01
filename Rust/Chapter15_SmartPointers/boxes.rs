// example cons list
enum List {
    Cons(i32, Box<List>), // for this recursive data structure, we use a pointer
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5); // storing a 5:i32 on the heap

    // ex. (1,(2,(3,Nil)))
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

