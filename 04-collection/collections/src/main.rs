use std::collections::HashMap;

fn main() {
    collection();
    string();
    map();
}

fn collection() {
    let mut vec: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3]; // 使用vec!宏可以让类型推断自己确认泛型类型
    println!("{}", vec2[0]);
    vec2.push(6);

    vec.push(20);
    println!("{}", vec.get(0).unwrap()); // get的返回值是一个Option

    // iterate + update
    for i in &mut vec2{
        *i += 20;
    }

    // iterate
    for i in &vec2 {
        print!("{} ", i);
    }
    println!("{}", vec2[0]);
    
}

fn string() {
    println!("===== String =====");
    let s = "hello，你好"; 
    println!("{}", s.len()); // 14，因为每个字符使用的字节数不同, 所以rust不支持索引访问字符串

    // 遍历字符串
    for i in s.chars() {
        println!("{}", i);
    }
}

fn map() {
    println!("===== HashMap =====");
    let mut map = HashMap::new();
    map.insert(String::from("hello"), String::from("world"));
    println!("{}", map.get("hello").unwrap());
    
    map.insert(String::from("hello"), String::from("what the hell"));
    println!("{}", map.get("hello").unwrap());

    // 类似putIfAbsent
    map.entry(String::from("mikoto")).or_insert("one last kiss".to_string());

    println!("{}", map.get("mikoto").unwrap());
}
