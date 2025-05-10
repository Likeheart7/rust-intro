mod average_collection;
mod blog;

use average_collection::AverageCollection;
use crate::blog::Post;

fn main() {
    // encapsulation();
    // inherit();
    // polymorphism();
    oop();
}

fn oop() {
    let mut post = Post::new();
    post.add_text("Someone became a orphan, his parents are about to divorced.");
    assert_eq!("", post.content());
}

/// 通过trait实现
fn polymorphism() {
}

/// 通过trait兼容
fn inherit() {

}

// 封装
fn encapsulation() {
    let mut collection = AverageCollection::new();
    collection.add(1);
    collection.add(2);
    collection.add(3);
    collection.add(4);
    // println!("{}", collection.average); // 无法访问
    println!("{}", collection.average()); // 正确
}
