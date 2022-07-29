use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess: ");
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //"shadow" guess variable rather than creating a new one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
