struct Fibonacci{
    curr: u64,
    prev: u64,
}

impl Fibonacci{
    fn new()-> Self{
        Fibonacci{
            curr:1,
            prev:0,
        }
    }
}

impl Iterator for Fibonacci{
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item>{
        let new_val = self.curr;
        let new_curr = self.curr+self.prev;
        self.prev = self.curr;
        self.curr = new_curr;
        Some(new_val)
    }
}

fn main() {
    let lib = Fibonacci::new();

    for (k,v) in lib.take(10).enumerate(){
        println!("Fib{}:{}", k,v);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci(){
        let fib = Fibonacci::new();
        let result: Vec<u64> = fib.take(10).collect();
        let expected = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        assert_eq!(result, expected);
    }
}