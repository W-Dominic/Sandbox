pub mod testing;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}


// testing integration tests
pub fn mult_two(val: i32) -> i32 {
    val * 2
} 


#[cfg(test)]
mod tests {
    use super::*; // for putting Rectangle into scope

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    /*
    #[test]
    fn fails(){
        panic!("This test fails!")
    }
    */
    #[test]
    fn test_rec(){
        let rec0 = Rectangle{width:3, height:4};
        let rec1 = Rectangle{width:2, height:2};

        assert!(rec0.can_hold(&rec1));
        assert!(!rec1.can_hold(&rec0));
    }

    #[test]
    #[should_panic] //tests for panic!
    fn greater_than_100() {
        Guess::new(200);
    }

    //using if statement and Result<> to prevent panic when failure
    #[test]
    fn it_works_noPanic() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
