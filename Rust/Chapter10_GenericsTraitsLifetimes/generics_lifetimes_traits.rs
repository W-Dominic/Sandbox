use std::fmt::Display; 

fn longest_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
    ) -> &'a str where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 : String = String::from("aaaaaaaaa");
    let s2 : String = String::from("aaaaaaaaaaaaaaa");

    let result: &str = longest_announcement(s1.as_str(), s2.as_str(), 
                                            "The longest string is: ");
}
