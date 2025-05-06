fn main() {
    shadow();
    owner();
}

// 遮蔽允许对同名变量更改类型
fn shadow() {
    let spaces = "     ";
    println!("{}", spaces);
    let spaces = spaces.len();
    println!("{}", spaces);
}

fn owner() {
    let word = String::from("hello");
    // let x = word;
    // // 报错，因为所有权转移给了x，导致word变量直接失效
    // println!("{}", word);

    // 可以使用，因为y只是借用了word的对象，
    // 可以认为y实际指向变量word而不是word指向的对象，所以不拥有所有权
    let y = &word;
    println!("{}", word);
}