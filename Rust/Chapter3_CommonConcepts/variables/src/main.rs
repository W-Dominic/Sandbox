use std::io;

fn main() {
    //shadowing 
    let word = "hello";
    let word = word.len();

    //expressions
    let y = {
        let x = 3;
        x + 1
    };

    //arrays
    let a = [1,2,3,4,5]; //length is immutable
    let a2: [i32; 5] = [0,1,2,3,4]; //specify type/length
    let a3 = [3,5]; //[3,3,3,3,3]


}
