//declare an enum for ip type
enum IpAddrKind {
    V4,
    V6,
}
/*
// approach using structs
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/
// more efficient approach using enums
// each enum name acts as a constructor function
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


//another enum example
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //something
    }
}
//implementation of "null" (included)
enum Option<T> {
    None,
    Some(T),
}
fn main() {
    /*
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    */

    /*
    let m = Message::Write(String::from("hello"));
    m.call();
    */

    //using "null"
    let some_number = Some(5);
    let some_char = Some('e');
    // Some(T) lets the compiler know that this field may be null
    // You cant do operations with Some(T) and T 
    // until you convert to T 

    let absent_number: Option<i32> = None;

}





