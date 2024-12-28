fn main() {
    test_lifetime();
}

#[test]
fn test_lifetime() {
    let large = longest("a","ab");
    println!("The longest is {}", large);

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        }else {
            y
        }
    }
}
