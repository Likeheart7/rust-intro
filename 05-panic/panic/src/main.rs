use std::{fmt::Display, fs::{read_to_string, File}, io::ErrorKind};
fn main() {
    // panic();
    result();

    let p = People{name: "chen".to_string(), age:23};
    dbg!("{}", p);
}

#[derive(Debug)]
struct People {
    name: String,
    age: u32,
}

fn panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn result() {
    let file = match File::open("./Cargo.toml") {
        Ok(file) => file,
        Err(_) => {
            panic!("Error: File not exist");
        }
    };

    let f = match read_to_string("./Cargo.oml") {
        Ok(content) => {
            println!("{}", content);
            content
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./hello.txt") {
                Ok(_) => String::from("File not found, a new file created"),
                Err(_) => panic!("File not found and create failed")
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        },
    };

    println!("{}", f);
}