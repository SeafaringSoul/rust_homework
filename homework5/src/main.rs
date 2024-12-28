fn main() {
    match call(2,6) {
        Ok(answer) => println!("The answer is {}", answer),
        Err(e) => println!("Error: {}", e)
    }
}

fn call (a:i32, b:i32) -> Result<f64,String> {
    match divide(a,b) {
        Some(result) => match sqrt(result) {
            Ok(sqrt_result) => Ok(sqrt_result),
            Err(e) => Err(format!("Error in sqrt: {:?}", e)),
        }
        None => Err("Divide by zero".to_string())
    }
}

fn divide (a:i32, b:i32) -> Option<f64> {
    if b != 0 {
        Some(a as f64 / b as f64)
    }else {
        None
    }
}
#[derive(Debug)]
pub enum MathError {
    DivideByZero,
    NegativeSquareRoot,
}

fn sqrt (x:f64) -> Result<f64,MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    }else {
        Ok(x.sqrt())
    }
}
