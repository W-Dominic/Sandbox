struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer { // custom code for what should happen when struct goes out of scope
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my Stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other Stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c); // use std::mem::drop if you want to drop early
    // d is taken out of scope automatically
}