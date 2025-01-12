use my_first_lib_crate::{math,User};

#[test]
fn test_math() {
    assert_eq!(math::add(1,2), 3);
    assert_eq!(math::sub(3,2), 1);
}

#[test]
fn test_user_integration() {
    let user = User::new("Bob".to_string(),18);
    assert_eq!(user.get_name(),"Bob".to_string());
    assert_eq!(user.get_age(),18)
}
