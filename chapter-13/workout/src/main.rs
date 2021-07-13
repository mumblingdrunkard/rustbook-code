use std::thread;
use std::time::Duration;
use workout::Cacher;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: &u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_millis(2000));
        num.clone().to_owned()
    };

    let mut cacher = Cacher::from(expensive_closure);

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(&intensity));
        println!("Next, do {} situps!", cacher.value(&intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", cacher.value(&intensity));
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    println!("Hello, world!");
}
