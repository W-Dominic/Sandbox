use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("test.txt");
    
    /*
    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {:?}", error),
    };
    */
    //handling multiple types of errors
    //does the file open?
    let file = match file_result {
        Ok(file) => file,
        //okay why doesnt it open? 
        Err(error) => match error.kind() {
            //doesnt exist? can we create it? 
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error);
            }
        }
    };
    
    //shortcut: if result ok, put in variable, else panic
    let fd1 = File::open("test.txt").unwrap(); 
    let fd2 = File::open("doesntexist.txt").unwrap(); 
    let fd3 = File::open("alsodoesntexist.txt")
        .expect("This file does not exist"); //expect lets you choose the error message >> best
}
