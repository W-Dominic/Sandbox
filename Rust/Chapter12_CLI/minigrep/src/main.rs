use std::env; // for cli args
use std::fs; // for reading from files

fn main() {
    let args: Vec<String> = env::args().collect();

    // check for valid number of args
    if args.len() != 3 {
        panic!("Invalid Number of Args")
    }

    // store args in variables
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for `{}`", query);
    println!("In file `{}`", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file");

    println!("With text:\n{contents}");
}
