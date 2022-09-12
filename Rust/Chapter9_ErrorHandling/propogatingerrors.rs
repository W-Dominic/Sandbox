use std::fs::File;
use std::fs::read_to_string;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("usernames.txt");
    //let username_file_result = File::open("test.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut(filename : &str) -> Result<String, io::Error> {
    let mut username_file = File::open(filename)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn even_simpler(filename : &str) -> Result<String, io::Error> {
    read_to_string(filename)
}


fn main() {
    let res = read_username_from_file();
    match res {
        Ok(username) => println!("Username is: {}", username),
        Err(e) => println!("Result: {:?}", e),
    }
    let res2 = read_username_from_file_shortcut("test.txt");
    match res2 {
        Ok(username) => println!("Username is: {}", username),
        Err(e) => println!("Result: {:?}", e),
    }
    let res3 = even_simpler("3121231312.txt");
    println!("{:?}", res3);
}
