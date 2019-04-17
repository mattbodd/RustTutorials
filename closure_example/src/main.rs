use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
   fn new(calculation: T) -> Cacher<T> {
        Cacher {
           calculation,
           value: None,
        }
   }

    // notice here that if value is Some, use what is stored inside, otherwise
    // recompute value
    fn value(&mut self, arg: u32) -> u32 {
       match self.value {
           Some(v) => v,
           None => {
               let v = (self.calculation)(arg);
               self.value = Some(v);
               v
           },
        }
    }
}


fn main() {    
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // showcase closure ability to capture environment
    let x = 4;

    // as we are just reading x, we only need to borrow x immutably so this closure has
    // Fn trait
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
