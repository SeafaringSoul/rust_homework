use my_first_lib_crate::math::add;

#[test]
fn example_1_2() {
    let sum = add(1, 2);
    assert_eq!(sum, 3);
}

fn main() {
    let sum = add(1, 1);
    assert_eq!(sum, 2);
    println!("sum is: {}", sum);
}
