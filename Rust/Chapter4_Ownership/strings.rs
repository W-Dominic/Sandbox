use std::io;

fn main(){
    let s0 = String::from("testing"); //string on the heap

    let mut s1 = String::from("hello"); //mutable string
    s1.push_str(", world!"); 
    println!("{}", s1);
}