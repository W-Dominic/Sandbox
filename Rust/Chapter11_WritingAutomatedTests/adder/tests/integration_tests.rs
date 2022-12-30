use adder::testing;
use adder;
#[test]
fn mult_works() {
    assert_eq!(16, adder::mult_two(8));
}

#[test]
fn div_works() {
    assert_eq!(4, testing::div_2(8));
    assert_eq!(2, testing::div_2(5));
}
