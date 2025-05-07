fn main() {
    let s1 = String::from("hello");
    let s2 = String ::from ("zbcde");
    println!("{}", longest(s1.as_str(), s2.as_str()))
}

// 通过生命周期，表示两个形参至少拥有一个重叠的生命周期，那么以两个中生命周期较短的作为返回值的生命周期
// 从而避免返回值作为引用，存活超过其所有引用的变量的生命周期从而变成悬垂指针
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1 > str2 {
        str1
    } else {
        str2
    }
}
