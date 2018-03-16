use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // let example_closure = |x| x;

    // let s = example_closure(String::from("hello")); //string
    // let n = example_closure(5);//u32 --> ERROR
    
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("2 seconds... calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let buceta = expensive_closure(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", buceta);
        println!("Next, do {} situps!", buceta);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", buceta);
        }
    }
}
