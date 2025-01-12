use std::time::Instant;
fn main() {
    // task1
    // stack_test();
    // heap_test();
    // task2 创建根文件夹
    let mut root = FolderNode {
        name: "root".to_string(),
        contents: Vec::new(),
    };

    // 添加文件和文件夹到根文件夹
    root.create_file("file1.txt");
    root.create_folder("folder1");

    // 在子文件夹中添加文件
    if let Some(folder1) = root.contents.iter_mut().find_map(|node| match node.as_mut() {
        Node::Folder(folder) => Some(folder.as_mut()),
        _ => None,
    }) {
        folder1.create_file("file2.txt");
        folder1.create_folder("folder2");

        // 在更深的子文件夹中添加文件
        if let Some(folder2) = folder1.contents.iter_mut().find_map(|node| match node.as_mut() {
            Node::Folder(folder) => Some(folder.as_mut()),
            _ => None,
        }) {
            folder2.create_file("file3.txt");
        }
    }

    // 列出文件系统结构
    root.list_contents(0);
}



fn stack_test(){
    // 测试栈上分配
    let start = Instant::now(); // 记录开始时间，开始测量栈分配时间
    let start_arr = [0u8;1_000_000]; // 创建一个包含100万个元素的栈上数组
    let stack_allocation_time = start.elapsed(); // 计算栈分配的耗时

    let start = Instant::now();
    let mut start_sum = 0;  // 用来存储栈上数组元素的和
    for &val in start_arr.iter() {  // 遍历栈上数组中的每个元素
        start_sum += val; //便利栈上的数组
    }
    let stack_access_time = start.elapsed();

    // 打印栈和堆的分配及访问时间
    println!("stack is time: {:?}", stack_allocation_time);
    println!("stack access: {:?}", stack_access_time);
}

fn heap_test(){
    //测试堆上分配
    let start = Instant::now();
    let heap_arr = Box::new([0u8;1_000_000]); // 创建一个包含100万个元素的堆上数组
    let heap_all_time = start.elapsed();

    let start = Instant::now();
    let mut heap_sum = 0;
    for &val in heap_arr.iter() {
        heap_sum += val;
    }
    let heap_acc_time = start.elapsed();
    println!("heap is time: {:?}", heap_all_time);
    println!("heap access: {:?}", heap_acc_time);
}

// 定义文件系统的节点
enum Node{
    File(String),
    Folder(Box<FolderNode>),
}

//定义文件夹结构
struct  FolderNode {
    name: String,
    contents: Vec<Box<Node>>,
}

// 实现一个文件系统
trait FileSystem{
    //创建文件
    fn create_file(&mut self,name: &str);
    // 创建文件夹
    fn create_folder(&mut self, name: &str);
    // 列出文件夹内容
    fn list_contents(&self,depth: usize);
}

impl FileSystem for FolderNode {
    fn create_file(&mut self, name: &str) {
        self.contents.push(Box::new(Node::File(name.to_string())));
    }

    fn create_folder(&mut self, name: &str) {
        self.contents.push(Box::new(Node::Folder(Box::new(FolderNode{
            name: name.to_string(),
            contents: Vec::new(),
        }))));
    }

    fn list_contents(&self,depth: usize) {
        let indent = "    ".repeat(depth);
        println!("{}Folder:{}", indent, self.name);
        for node in &self.contents {
            match &**node {
                Node::File(name) => println!("{} File: {}", indent,name),
                Node::Folder(folder) => folder.list_contents(depth+1),
            }
        }
    }
}
