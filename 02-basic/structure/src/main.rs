fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("the width is {:#?}", rec);
    dbg!(&rec);
    println!("area of this rectangle is {}", rec.area());
    println!("If rec can other rectangle {}", rec.can_hold(&Rectangle {
        width: 20, 
        height: 60
    }));

    dbg!(&Rectangle::square(3));
    println!("{}", &Tuple(10, 20, 30).1)    // 访问元组结构体的属性
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// 给结构体定义方法的方式
impl Rectangle {
    fn area(&self) -> u32 { // 类似python实例方法的第一个参数，这里是借用
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height   
    }
    
    // 可以理解成静态函数，通过::调用
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 元组结构体
struct Tuple(i32, i32, i32);