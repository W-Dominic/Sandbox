use std::collections::HashMap;

fn twosum (v: Vec<i32>, target : &i32) -> (i32, i32) {
    //keep track of <num, index>
    let mut hmap : HashMap<i32, i32> = HashMap::new();
    
    let mut counter = 0;
    for i in v {
        let sub : i32 = target - i;
        if !hmap.contains_key(&sub) {
            hmap.insert(i,counter);
        }
        else {
            return (i, sub)
        }  
        counter += 1; 
    }

    (-1, -1)

}

fn main(){
    //let's do twosum!
    let v : Vec<i32> = vec![1,2,3,4,5];
    let target = 9;

    let (num1, num2) = twosum(v, &target);
    println!("{} plus {} equals {}", num1, num2, target);
}
