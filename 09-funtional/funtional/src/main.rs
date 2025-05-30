use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_work(simulated_user_specified_value, simulated_random_number);
}

fn generate_work(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|intensity| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today. Remember to stay hydrated!")
        } else {
            println!("Today, run for {} minutes", expensive_result.value(intensity))
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}