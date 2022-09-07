fn plus_one(x: Option<i32>) -> Option<i32> {
        //if x is null then the case gets handled
        match x {
            None => None,
            Some(i) => Some(i + 1),
            // _ => (), //catch all, nothing happens
        }
    }
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
