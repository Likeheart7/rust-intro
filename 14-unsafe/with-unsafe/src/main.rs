use std::ops::Add;
use std::{fmt, slice};
use std::fmt::Formatter;

fn main() {
    basic();
    ffi();
    access_mut_static_variable(3);
    operator_overload();
    impl_foreign_trait();
}

static mut COUNTER: i32 = 0;

/// 访问可变的静态变量
fn access_mut_static_variable(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
    unsafe {
        println!("COUNTER is: {}", COUNTER);
    }
}

fn basic() {
    let mut num = 5;
    let r1 = &num as *const i32;    // 裸指针
    let r2 = &mut num as *mut i32;
    // 创建裸指针是安全的，但是访问是不安全的
    // println!("{}", *r1); // Error
    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    // cannot borrow `*slice` as mutable more than once at a time
    // 不能同时对一个对象持有多个可变引用，返回值里持有了两个，即使是对不同切片的引用，但是编译器无法确定
    // (&mut slice[..mid], &mut slice[mid..])
    let ptr = slice.as_mut_ptr();
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len-mid))
    }
}

unsafe fn dangerous() {
    // 一个unsafe 方法
}



/// 访问外部函数的方式
fn ffi() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 声明将调用C语言的abs方法
extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle] // 指定编译器不要打乱方法的名称，以便其他语言查找
pub extern "C" fn call_from_c() {
    println!("交给其他语言调用的方法")
}


#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point; // 指定该实现的返回类型，不使用泛型是因为泛型会导致可以为一个类型实现多次不同泛型参数的

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn operator_overload() {
    let p1 = Point {
        x: 1,
        y: 2
    };
    let p2 = Point {
        x: 10,
        y: 20
    };
    assert_eq!(p1 + p2, Point {x: 11, y: 22})
}


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn impl_foreign_trait() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}