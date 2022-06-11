use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 创建线程池。
    ///
    /// size：线程池中线程的数量。
    ///
    /// # Panics
    /// `new`函数会在size为0时触发panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // 创建 channel，channel 可以在 CSP 模型中交换数据。
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // 创建指定大小的数组。
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 在堆上申请空间，并将任务地址发送给 Woker。
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 通知线程池中所有的线程停止运行。
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        // 然后将还在运行的线程移出线程池，并使用 join 方法继续执行，执行后会自动退出。
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // 先上锁，然后读取消息。
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        // 创建新的 Woker，并将 Woker 返回。
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
