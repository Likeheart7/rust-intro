fn main() {
    if_let();
}

fn if_let() {
    let favorite_color: Option<&str> = None;
    let flag = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}", color);
    } else if flag {
        println!("this across flag");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("You are not young anymore");
        } else {
            println!("Enjoy your life.")
        }
    } else {
        println!("I can do nothing for you.");
    }
}
