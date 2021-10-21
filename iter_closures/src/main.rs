use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specidied_value = 10;
    let simulated_random_number = 7;

    // generated_workout4(simulated_user_specidied_value, simulated_random_number);
    // generated_workout4(5, 7);

    catch_variables();
    // catch_variables2();
    catch_variables3();

    // test();
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generated_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Tiday, do {} pushups",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!(
                "Today, run for {}",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// Refactoring with functions
fn generated_workout2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

fn generated_workout3(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break!")
        } else {
            println!(
                "Today run for {} miles",
                expensive_closure(intensity)
            );
        }
    }
}

fn add_one_v1(x: u32) -> u32 {
    let add_one_v2 = |x: u32| -> u32 {
        x + 1
    };
    // let add_one_v3 = |x| {
    //     x + 1
    // };
    // let add_one_v4 = |x| x + 1;
    x + 1
}

fn test() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

// struct Cacher<T>
//     where
//         T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }

struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // let calc = &self.calculation;
        // self.value.entry(arg).or_insert_with(|| calc(arg))

        if self.value.contains_key(&arg) {
            self.value[&arg]
        } else {
            self.value.insert(arg, (self.calculation)(arg));

            self.value[&arg]
        }

        // match self.value {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //
        //         v
        //     }
        // }
    }
}

fn generated_workout4(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));

        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            print!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

fn catch_variables() {
    let x = 4;

    let equal_to_x = |z| z == x; // Имеет типаж Fn, т.к. тело замыкание должно только читать знач. в x

    let y = 4;

    assert!(equal_to_x(y));
}

// fn catch_variables2 () {
//     let x = 4;
//
//     fn equl_to_x(z: i32) -> bool {
//         z == x
//     }
//
//     let y = 4;
//
//     assert!(equl_to_x(y));
// }

fn catch_variables3 () {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}