use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// 定义任务结构
struct Task{
    id:usize,
    job:Box<dyn FnOnce() +Send + 'static>
}
//定义调度器结构
struct Scheduler{
    task_queue: Arc<Mutex<VecDeque<Task>>>,
    workers:Vec<thread::JoinHandle<()>>,
    sender:mpsc::Sender<(usize,String)>,
}
impl Scheduler{
    // 构造函数，初始化线程池和任务队列
    fn new(worker_count:usize)->(Self,mpsc::Receiver<(usize,String)>){
        let task_queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));
        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let task_queue = Arc::clone(&task_queue);
            let sender = sender.clone();

            workers.push(thread::spawn(move || loop {
                let task = {
                    let mut queue = task_queue.lock().unwrap();
                    queue.pop_front()
                };

                if let Some(task) = task {
                    // 执行任务并发送结果
                    (task.job)();
                    sender.send((task.id, format!("Task {} completed", task.id))).unwrap();
                } else {
                    break;
                }
            }));
        }

        (
            Scheduler {
                task_queue,
                workers,
                sender,
            },
            receiver,
            )
    }

    // 添加任务 到队列
    fn add_task<F>(&self, id: usize, job: F)
    where F: FnOnce() + Send + 'static
    {
        let mut queue = self.task_queue.lock().unwrap();
        queue.push_back(Task{id,job: Box::new(job)});
    }

    // 启动调度器
    fn start(&self){

    }

    fn wait(self){
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}

fn main() {
    println!("Hello, world!");
    let (scheduler,receiver) = Scheduler::new(4);

    for i in 1..=5 {
        scheduler.add_task(i, move|| {
            println!("Executing task {}", i);
            thread::sleep(Duration::from_millis(500));
        })
    }

    scheduler.start();

    for result in receiver{
        println!("{}", result.1);
    }

    scheduler.wait();
}
