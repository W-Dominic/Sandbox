fn main() {
    let s = String::from("Hello");
    
    takes_ownership(s);

    let x = 5; 
    
    makes_copy(x);

    
    // println!("{}", s); //fails, s is no longer in scope
    println!("{}", x); //works because the function makes a copy of x
}

fn takes_ownership(s : String) {
    println!("{}", s);
}

fn makes_copy( i : i32){
    println!("{}", i);

}
