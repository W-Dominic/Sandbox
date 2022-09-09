fn main() {
    //specify type because its empty originally
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(3);
    v1.push(3);
    v1.push(7);
    //no need to specify type
    let v2 = vec![1, 2, 3];

    //access element using get()
    let element = v1.get(0);
    match element {
        Some(element) => println!("Exists"),
        None => println!("Does not exist"),
    }
    println!(
        "Different ways to access vectors elements
        {}, {}"
        , v1[0], &v1[0]
        );

    //iterating over vectors
    for i in &v2 {
        println!("{}", i);
    }
    //iterating over mutable references to each element
    for i in &mut v1 {
        *i += 1000;
        println!("{}", *i);
    }

    //vector of multiple types
    enum multiple {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        multiple::Int(5),
        multiple::Float(5.55),
        multiple::Text(String::from("5")),
    ];
}
