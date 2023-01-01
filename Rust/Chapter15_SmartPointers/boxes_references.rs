fn main() {
    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);
    let z: Box<i32> = Box::new(x); // we can have multiple pointers to the same value

    assert_eq!(5,x);
    assert_eq!(5,*y); // Boxes implement the Deref trait
    assert_eq!(5,*z);

    // deref coercion example
    let m = Box::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str){
    println!("hello! {name}");
}