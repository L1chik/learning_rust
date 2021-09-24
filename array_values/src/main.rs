use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        let mut list_input = String::new();
        println!("Input list of int:");

        io::stdin()
            .read_line(&mut list_input)
            .expect("Failed to read line");

        let trimmed = list_input.trim();
        if trimmed == "quit" {
            break;
        }

        let mut arr: Vec<i32> = trimmed
            .split(',')
            .map(|x| match x.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        })
            .collect();

        arr.sort();

        let mut result = Values {
            mean: mean(&arr),
            median: median(&arr),
            mode: mode(&arr),
        };

        println!("Sorted array: {:?}. Mean: {}, Median: {}, Mode: {}", arr, result.mean, result.median, result.mode);
    }
}


struct  Values {
    mean: f64,
    median: f64,
    mode: i32,
}

fn mean(arr: &[i32]) -> f64 {
    let sum:i32 = Iterator::sum(arr.iter());
    f64::from(sum) / (arr.len() as f64)
}

fn median(arr: &Vec<i32>) -> f64 {
    let len = arr.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&arr[(mid - 1)..(mid + 1)])
    } else {
        f64::from(arr[mid])
    }
}

fn mode(arr: &[i32]) -> i32 {
    let mut map: HashMap<&i32, i32> = HashMap::new();

    for it in arr {
        let count = map.entry(it).or_insert(0);
        *count += 1;
    }

    let mut max = (0, 0);
    for (&&key, &val) in &map {
        if val > max.1 {
            max = (key, val);
        }
    }

    max.0
}