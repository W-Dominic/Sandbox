fn main() {
    let mut stack: Vec<char> = vec!['t', 's', 'u', 'r'];
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

}
