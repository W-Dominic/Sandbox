//ex1. coin sorting
#[derive(Debug)]
enum UsState {
    NewJersey,
    Florida,
    NewYork,
}
enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    //compiler checks if all possible cases are handled
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>  {
            println!(
                "State Quarter from {:?}!", 
                state
            );
            25 //returns last value
        }
        // _ => 99, //wildcard
        
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Florida));
}
