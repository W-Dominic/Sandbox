fn main() {
    
    //ex.1
    let config_max = Some(3u8); //3u8 -> 3:u8
    //if-let opposed to match
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    //if-let is not exhuastive like match ... but you write less code

    //ex.2 else

    let null_max : Option<i32> = None;
    if let Some(max) = null_max {
        println!("Not null");
    }
    else {
        println!("Cannot assign null value");
    }

}
