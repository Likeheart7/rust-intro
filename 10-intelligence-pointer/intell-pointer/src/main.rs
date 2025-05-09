use std::ops::Deref;
use std::rc::Rc;
use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};

fn main() {
    with_box();
    custom_deref();
    reference_count();
    ref_cell();
}


// Rc<T> 用于引用计数，从而将所有权交给多方共享
fn reference_count() {
    let a = Rc::new(RcCons(5, Rc::new(RcCons(6, Rc::new(RcNil)))));
    let b = RcCons(3, Rc::clone(&a)); // Rc::clone不会深拷贝，只会增加引用计数，所以不会有太多开销
    let c = RcCons(3, Rc::clone(&a));
    {
        let d = RcCons(4, Rc::clone(&a));
        println!("count = {}", Rc::strong_count(&a));
    }
    println!("total count = {}", Rc::strong_count(&a));
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

fn with_box() {
    // Box包装的数据分配在堆上，一般用于防止复制的开销
    let num = Box::new(5);
    println!("{}", num);

    // 递归类型
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn custom_deref() {
    let my_box = MyBox::new(5);
    println!("the value in my box is: {}", *my_box);
}


// RefCell用于处理内部可变性
// 默认情况下，rust在编译期间要求数据有唯一的可变引用或多个不可变引用，ReCell可以将其延迟到运行时检查
fn ref_cell() {
    println!("=====>>> ReCell <<<=====")

}

pub trait Messager{
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messager {
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    struct MockMessager {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let messager = MockMessager::new();
        let mut tracker = LimitTracker::new(&messager, 100);
        tracker.set_value(80);
        assert_eq!(messager.sent_messages.borrow().len(), 1);
    }
}