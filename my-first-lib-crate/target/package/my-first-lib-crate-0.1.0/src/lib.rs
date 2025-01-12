pub mod math{
    /// Adds two numbers.
    ///
    ///  # Examples
    ///
    /// ```
    /// use my_first_lib_crate::math::add;
    /// assert_eq!(add(2,3),5);
    /// ```
    pub fn add(a: i32, b: i32) -> i32{
        a + b
    }

    /// fist number - second number
    pub fn sub(a: i32, b: i32) -> i32{
        a - b
    }
}

pub struct User{
    name: String,
    age: u32,
}

impl User{
    pub fn new(name: String, age: u32) -> Self{
        Self{name:name.to_string(), age}
    }

    /// Returns the user's name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the user's age.
    pub fn get_age(&self) -> u32 {
        self.age
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add(){
        assert_eq!(math::add(2, 2), 4);
    }

    #[test]
    fn test_sub(){
        assert_eq!(math::sub(5, 2), 3);
    }

    #[test]
    fn test_user_creation(){
        let user = User::new("John Doe".to_string(), 18);
        assert_eq!(user.name, String::from("John Doe"));
        assert_eq!(user.age, 18);
    }
}