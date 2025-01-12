use std::{fs, thread};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

// 作业1:文件处理函数
fn process_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(content) => println!("文件:{}\n内容:{}", file_path, content),
        Err(error) => println!("Error:\n{}", error),
    }
}

// 作业2 ：
enum Task{
    Work(String), // 普通任务，包括任务函数
    Stop,
}

//定义线程池
struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>, //祝线程发送任务的通道
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    //创建工作线程
    fn new(id: usize, result: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            // 从通道中接收任务
            let task = result.lock().unwrap().recv().unwrap();
            match task {
                Task::Work(description) => {
                    println!("Worker {} is processing task: {}",id,description);
                    thread::sleep(Duration::from_millis(1));
                }
                Task::Stop => {
                    println!("Worker {} received stop signal",id);
                    break;
                }
            }
        });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}

impl ThreadPool {
    //创建线程池
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); //包装接收端，线程安全

        // 创建工作线程
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{workers, sender}
    }

    // 发送任务到线程池
    fn execute(&self, task: Task){
        self.sender.send(task).unwrap();
    }

    //停止所有线程
    fn stop_all(self){
        //发送停止信号给所有线程
        for _ in &self.workers {
            self.sender.send(Task::Stop).unwrap();
        }

        // 等待所有线程退出
        for worker in  self.workers {
            if let Some(handle) = worker.handle {
                handle.join().unwrap();
            }
        }
    }
}

fn main() {
    // 作业1
    // // 创建 缓冲的 channel （限制 4个）
    // let (tx,rx): (Sender<String>,Receiver<String>) = mpsc::channel();
    //
    // // 接收器包装 mutex 县城共享
    // let rx = Arc::new(Mutex::new(rx));
    //
    // //创建线程池，4个
    // let thread_count = 4;
    // let mut workers = vec![];
    //
    // for _ in 0..thread_count {
    //     let rx = Arc::clone(&rx);
    //     workers.push(thread::spawn(move || {
    //         while let Ok(file_path) = rx.lock().unwrap().recv() {
    //             process_file(&file_path);
    //         }
    //     }))
    // }
    // //模拟发送
    // let file_paths = vec![
    //     "file1.txt",
    //     "file2.txt",
    //     "file3.txt",
    //     "file4.txt",
    //     "file5.txt",
    //     "file6.txt",
    //     "file7.txt",
    //     "file8.txt",
    //     "file9.txt",
    //     "file10.txt",
    // ];
    //
    // for path in file_paths {
    //     tx.send(String::from(path)).unwrap();
    // }
    //
    // // 关闭发送端
    // drop(tx);
    //
    // // 等待所有线程完成
    // for worker in workers {
    //     worker.join().unwrap();
    // }

    //作业2
    let pool = ThreadPool::new(4);

    for i in 1..=10{
        pool.execute(Task::Work(format!("Task {}",i)));
    }

    pool.stop_all();
}




任务插述
。你需要编写一个简单的多线程任务调度品，亡能够接收多个任务，井将这兴任务分发到多个工作线程中热行。调度器使用Channel进行任务的分发和结果的收集。你需要使用Rust的send和sync特性来确保任务调度器在多线程环境中的安全性。
•具体要求
≤任务结构
。定义一个Task结构体，表示需要执行的任务。任务包含一个唯一的id和一个用手执行的闭包。
•调度器结构
。创建一个Scheduler结构体，包含一个任务队列和一个线程池。调度路应当使用channel来分发任务到不同的工作线程。
功能实现：
•调厦器豆当具有以下功能
。添加任务：向调度器添加一个任务。
启动调度器：启动多个线程，开始从任务队列中获取任务井执行获取结果：在所有任务完成后，收集井打印每个任务的扶行结果。
多线程安全
通过使用ArC和Mutex确保任务队列在三个线程之自的安全访问
确保任务的结果能够王确地在线程之间传递和收集。
问题提示
•任务队列：
。使用Mutex来保护任务队列，确保多个线程不会同时修改队列中的数据。
。使用ArC来共享任务队列的所有权，使得多个线程能够访问同一个任务队列。
•任务分发：
。使用channel来将任务的完成状态发送回主线程，从而可以在主线程中收集和打印任务完成的结果。
•线程池：
。通过循环创建多个工作线程，每个线程从任务队列中取出任务并执行。线程池的大小可以通过Scheduler
的枸造西数来脂定。
•任务执行：
。每个任务都应该是一个用包，使用Box<dyn FnOnce0-将其存储在Task结构体中。