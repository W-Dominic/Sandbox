#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32, 
}

//rectangles associated functions (methods)
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect : &Rectangle) -> &str { 
        if rect.width < self.width && rect.height < self.height {
            return "Yes"
        }
        return "No"
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };
    println!(
        "Can 1 hold 2? {}",
        rect1.can_hold(&rect2)
    );

    //creating a square
    let sq = Rectangle::square(5);
    println!(
        "{:?}", sq
    );
}
