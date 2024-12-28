//方法1 & 取地址，避免所有权转移
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = takes_ownership(&s1);
//     println!("{}, world!", s1);
//     println!("{}, world!", s2);
// }
//
// fn takes_ownership(s: &String) -> String {
//     s.clone()
// }

// 方法2 函数返回所有权，在复制一份给s2
fn main() {
    let s1 = String::from("hello");
    let s1 = takes_ownership(s1);
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);
}

fn takes_ownership(s: String) -> String {
    s
}