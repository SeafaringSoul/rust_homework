use std::collections::HashMap;
use std::fmt;
use std::fmt::write;

fn main() {
    // task1
    // let mut stack = Stack::new();
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    //
    // println!("The first element is {}", stack.peek().unwrap());
    // println!("pop 出的元素 {}", stack.pop().unwrap());
    // println!("剩余元素: {}", &stack);

    // task2
    // let str = "hello world";
    // let now_str = count_chars(&str);
    // print_frequency(&now_str);

    // task3
    let mut stocks = Stocks::new();

    //添加书籍
    stocks.add_book("1-1".to_string(),"书籍1".to_string(),"简介1".to_string());
    stocks.add_book("1-2".to_string(),"书籍2".to_string(),"简介2".to_string());
    println!("添加成功！");
    stocks.all_books();
    // 修改书籍
    stocks.update_book("1-1",Some("书籍1-edit".to_string()),Some("简介 修改了".to_string()));
    println!("已修改：");
    stocks.all_books();
    // 删除书籍
    stocks.remove_book(&"1-2".to_string());
    println!("删除了：");
    stocks.all_books();
    // 查询书籍
    let (books,len) = stocks.search_book("书籍1-edit".to_string());
    println!("查询结果：{:?}, 库存：{}",books,len);
    // 当前所有书籍
    println!("当前所有书籍：");
    stocks.all_books();
}
// task1：使用Vec实现一个简单栈（后进先出，LIFO）数据结构，支持push，pop和peek操作。
struct Stack<T>{
    // 1. 声明一个空的结构体
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {elements: Vec::new()}
    }

    fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elements = &self.elements.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        write!(f,"[{}]",elements.join(", "))
    }
}

// task2：使用 HashMap 实现一个字频统计器，统计一个字符串每次出现的频率
fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        if c == ' ' {
            continue;
        }
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    map
}

fn print_frequency (map: &HashMap<char, usize>) {
    for (key,value) in map.iter() {
        print!("'{}': {} \n",key,value);
    }
}

// task3
#[derive(Debug)]
struct Book{
    title: String,
    dec: String,
}

struct Stocks{
    books: HashMap<String,Book>,
    id:Vec<String>,
}
//对结构体操作的方法
impl Stocks {
    fn new()-> Self{
        Stocks {
            books: HashMap::new(),
            id: Vec::new(),
        }
    }
    fn add_book(&mut self, id: String,title: String, dec: String) {
        let book = Book {title,dec};
        self.books.insert(id.clone(),book);
        self.id.push(id);
    }

    fn remove_book(&mut self, id: &String) {
        self.books.remove(id);
        //保留没有删除的id
        self.id.retain(|x| x != id);
    }

    fn update_book(&mut self, id: &str, title: Option<String>, dec: Option<String>) {
        if let Some(book) = self.books.get_mut(id) {
            if let Some(t) = title {
                book.title = t;
            }
            if let Some(c) = dec {
                book.dec = c;
            }
        }
    }
    // 查询书籍
    fn search_book(&self, title: String) -> (Vec<&Book>,usize) {
        let value_books: Vec<&Book> = self.books.values().filter(|x| x.title == title).collect();
        let count = value_books.len();
        (value_books, count)
    }

    // 查询所有信息
    fn all_books(&self) {
        if self.books.is_empty(){
            println!("No books found.");
        }else{
            for(i,v)in self.books.iter(){
                println!("书名：{}, 简介：{}",v.title,v.dec);
            }
        }
    }
}




