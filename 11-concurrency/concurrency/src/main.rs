use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    basics();
    message_between_thread();
    mutex();
    sync_send();
}

/// Sync和Send trait的可扩展并发
/// Send 标记 trait 表明类型的所有权可以在线程间传递。RC<T>不是
/// Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。
fn sync_send() {

}

/// 互斥锁
fn mutex() {
    // Mutex类似RefCell提供了内部可变性
    let mtx = Mutex::new(5);
    {
        let mut guard = mtx.lock().unwrap();
        *guard = 7;
        // 结束会调用drop方法自动释放锁
    }
    println!("m = {:?}", mtx);
    // 使用Arc在多线程之间共享
    println!("使用Arc在多线程之间共享数据");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..1000 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("The result is: {}", counter.lock().unwrap());
}

/// 通过mpsc::channel实现线程之间的通信
fn message_between_thread() {
    println!("=====>>> 线程之间的通信");
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move ||  {
        // let val = "hello".to_string();
        // // 会转走所有权
        // tx.send(val).unwrap_or_else(|e| println!("Something went wrong during send message. {}", e));
        // println!("{}", val); // Value used after being moved
        let greetings = vec!["hey", "hi", "hello"];
        for greeting in greetings {
            tx.send(greeting).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    thread::spawn(move ||  {
        let greetings = vec!["someone", "no one", "anyone"];
        for greeting in greetings {
            tx2.send(greeting).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    rx.into_iter().for_each(| message | { println!("Got: {}", message); });

}

fn basics() {
    let task1 = || {
        for i in 0..3 {
            println!("task1 now is: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    };
    let task2 = || {
        for i in 100..103 {
            println!("task2 now is: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    };
    let task1_handle = thread::spawn(task1);
    let task2_handle = thread::spawn(task2);
    task1_handle.join().unwrap();
    task2_handle.join().unwrap();

    println!("=====>>> with keyword `move`");
    let v = vec!["h", "e", "l", "l", "0"];
    let task_with_move = || {
        for i in  v {
            println!("{}", i);
        }
    };
    thread::spawn(task_with_move).join().unwrap();
}
