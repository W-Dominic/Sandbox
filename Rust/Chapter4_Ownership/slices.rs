fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    let word = first_word(&s[..]);

    let s_literal = "hello world";
    let word = first_word(s_literal); //literals are slices
    let word = first_word(&s_literal[..]);
    //s.clear(); //error
    
    println!("first word: '{}'", word);


    //arrays are slices too
    let a = [1,2,3,4,5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);

}

//string slices example
fn first_word(s: &str) -> &str {
    //string to array of bytes
    let bytes = s.as_bytes();

    //creates iterator over the array
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s //both &String and &str can be returned
}
