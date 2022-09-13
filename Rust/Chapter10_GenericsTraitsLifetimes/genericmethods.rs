struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    //returns point.x
    fn x (&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct BetterPoint<X1,Y1> {
    x: X1,
    y: Y1,
}
impl<X1,Y1> BetterPoint<X1, Y1> {
    fn mixup<X2,Y2>(self, other: BetterPoint<X2, Y2>) -> BetterPoint<X1, Y2> {
        BetterPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x:5, y:10 };
    let p2 = Point { x:5.5, y:4.4};

    println!("p.x = {}", p.x());
    //although you can just do this ->
    println!("p.x = {}", p.x);

    //method on specific type
    println!("Distance from origin = {}", p2.distance_from_origin());
    
    // this fails
    //println!("Distance from origin = {}", p.distance_from_origin());

    let point1 = BetterPoint { x: "hello", y: 5 };
    let point2 = BetterPoint { x: "goodbye", y: 10.5 };

    let point3 = point1.mixup(point2);
    println!("{:?}", point3);

}
