use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // 校验
        assert!(size > 0);
        // 用于线程间通信
        // 注意sender存在线程池，receiver存在Worker
        let (sender, receiver) = mpsc::channel();
        // 用于线程之间共享
        let receiver = Arc::new(Mutex::new(receiver));
        // 创建线程池，存放线程
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // 创建Worker并存入Vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    // 接收闭包，后续执行
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 将接收到的闭包(要执行的任务)用Box封装一下
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/// 为ThreadPool实现Drop确保停机前所有线程执行完任务
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all worker.");
        // 给每个Worker都发送Terminate请求
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // 通过join等待每个线程都执行完
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// 用于包装线程，持有id用于分辨
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // Worker中线程的工作，无限循环等待任务
        let thread = thread::spawn(move || loop {
            // 使用receiver前先获取锁，从receiver获取一个Message
            let message = receiver.lock().unwrap().recv().unwrap();
            // 是job就执行，是Terminate就结束循环
            // 因为如果接收到的是Terminate，这个Worker结束循环就不会再使用receiver，所以停止时发送的线程数个Terminate一定会被分配个每个线程一个
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing......", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                },
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// 使用该类型封装是为了让线程池区分待执行任务和终止请求
enum Message {
    NewJob(Job),
    Terminate,
}
