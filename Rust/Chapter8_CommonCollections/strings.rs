fn main() {
    let mut s = String::new();
    // let s = "initial".to_string();
    // s = String::from("inital contents");
    s.push_str("additional");
    // can work like a stack
    s.push('x');
    s.pop();
    println!("{}",s);

    //concatenation with +
    let s1 = "Hello".to_string();
    let s2 = "World".to_string();
    let s3 = s1 + &s2; //s1 has been moved and can no longer be used
    println!("{}",s3);

    //format!
    let st1 = String::from("tic");
    let st2 = String::from("tac");
    let st3 = String::from("toe");

    let res = format!("{}-{}-{}", st1, st2, st3);
    println!("{}",res);

    // string indexing
    // to index a string use string slices
    let hello = "Здравствуйте";
    let slice = &hello[0..4];
    println!("{}",slice); //prints Зд because of utf8 

    // iterating over strings
    // best practice is to specify chars or bytes
    for c in hello.chars() {
        println!("{}", c);
    }
    //example of bytes
    for b in hello.bytes() {
        println!("{}", b);
    }

}
