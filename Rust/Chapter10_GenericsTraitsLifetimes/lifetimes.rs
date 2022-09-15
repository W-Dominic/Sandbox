// by default variable exist in a predefined scope.
// lifetimes let you modify scope
// examples:
// & -> reference
// &'a -> reference with lifetime a
// &'a mut -> mutable reference with lifetime a
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn first<'a> (x: &'a str, y: &str) -> &'a str {
    x
}

fn main () {
    /*
    let string1 = String::from("abcd");
    let string2 = "xyzabcd";
        
    let longest_res = longest(string1.as_str(), string2);
    println!("The longest string is {}", longest_res);
    */
    let string4 = "bbbbbb";        
    let longest_res2;
    {
        let string3 = "aaa";
        longest_res2 = longest(string3, string4);
    }
    println!("The longest string is {}", longest_res2);

    /*
    let first_res = first(string1.as_str(), string2);
    println!("The first string is {}", first_res);
    */
}


/*
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
*/
