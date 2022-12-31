fn main() {
    // simple example
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Val: {}", val);
    }

    #[test]
    fn iterator_sum() {
        let v = vec![1,2,3];
        let v_iter = v.iter();
        let total: i32 = v_iter.sum();
        // .sum() consumes the iterator
        assert_eq!(total, 6);
    }

    // map example 
    let v2: Vec<i32> = vec![1,2,3];
    let v3 = v2.iter().map(|x| x + 1);
    for val in v3 {
        println!("New val: {:?}", val);
    }
}