struct Point<T> {
    x: T,
    y: T,
}

struct betterPoint<T,U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x:5.555, y: 4.999 };
    
    
    //let wont_work = Point { x:5, y:5.0 };
    // cant miss match types with this declaration
    
    //we can specify a new struct to allow for miss matched types
    let will_work = betterPoint { x:5, y:5.0 };
}
