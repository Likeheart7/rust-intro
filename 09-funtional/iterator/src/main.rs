fn main() {
    iterate();
    custom_iterator();
}

fn iterate() {
    let v = vec![1, 2, 2, 3];
    let iter = v.iter();
    // for val in iter {
    //     println!("{}", val);
    // }
    // println!("{}", iter.next().unwrap());
    let v2: Vec<_> = iter.map(|x| x + 3).collect();
    for val in v2 {
        println!("{}", val);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: "boot".to_string()
            }
        ]
    );
}

// 自定义迭代器

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn custom_iterator() {
    let mut counter = Counter::new();
    println!("{}", counter.next().unwrap());
    println!("{}", counter.next().unwrap());
    println!("{}", counter.next().unwrap());
    println!("{}", counter.next().unwrap());
    println!("=======");
    let counter2 = Counter::new();
    let result: u32 = counter2.zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .sum();

    println!("{}", result);
}