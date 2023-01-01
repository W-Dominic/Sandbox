// reference counting using rc<T>
// smart pointer which keeps track of num of references
// once last reference leaves, data can be cleaned up

// useful for graphs where multiple nodes may reference another node through edges
// all of these nodes have ownership of the connected node

// ONLY FOR SINGLE THREADED APPLICATIONS

// let's take our cons list for example
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc; // reference counted smart pointers


fn main() {
    /*
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(2, Box::new(a));

    this code wont compile because two Box<T> lists attempt to share ownership of a third list
    when b is declared a is moved into b
    c cannot access a because it is owned by b

    we need a rc<T> reference counted smart pointer in this scenario ...
    */

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // does not create a deep copy of a
                                    // only increases a reference count, which is quick
    let c = Cons(2, Rc::clone(&a));
    println!("Reference count on a: {}", Rc::strong_count(&a));
    assert_eq!(3, Rc::strong_count(&a));
}