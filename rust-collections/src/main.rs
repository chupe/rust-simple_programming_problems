// #![allow(unused, dead_code)]
extern crate rand;

use std::collections::HashMap;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut v = create_vector(100000, 1, 10000000);
    println!("Finding average, mode and median values of a list of ints.\n");
    // let mut v = vec![7, 2, 1, 8, 6, 3, 5, 4];
    let _median = median(&mut v);
    let _mode = mode(&v);
    let _average = average(&v);
}

fn median(v: &mut Vec<u32>) -> f32 {
    let _sorted = quicksort(v.as_mut_slice());
    let mut median_result: f32 = 0.0;
    if v.len() % 2 == 0 {
        let median_index = v.len() / 2 - 1;
        median_result = (&_sorted[median_index] + &_sorted[median_index + 1]) as f32 / 2.0;
    } else {
        let median_index = (v.len() + 1) / 2 - 1;
        median_result = _sorted[median_index] as f32;
    }
    // print_vector(&_sorted);
    println!("Median value is {}", median_result);
    median_result
}

fn quicksort(v: &mut [u32]) -> Vec<u32> {
    let v_len = v.len();
    let mut sorted: Vec<u32> = Vec::new();
    let pivot = v[v_len - 1];
    // Need to make i -1 but the type is usigned. Var first_time help alleviating the issue.
    let mut i: usize = 0;
    let mut first_time = true;
    let mut smaller_vec = Vec::new();
    let mut larger_vec = Vec::new();
    for j in 0..v_len - 1 {
        match v[j].cmp(&pivot) {
            Ordering::Less => {
                i += 1;
                // This is circumventing the fact I'm unable to make i = -1 to start with
                if first_time {
                    i -= 1;
                    first_time = false;
                }
                let temp = v[j];
                v[j] = v[i];
                v[i] = temp;
                // println!("{}, {}", i, j);
                let mut vec = Vec::new();
                vec.extend_from_slice(v);
                // print_vector(&vec);
            }
            _ => (),
        }
        if j == v_len - 2 {
            // println!("End of iteration cycle: {}, {}", i, j);
            let mut smaller: &[u32] = &[];
            let mut larger = &v[i..v_len - 1];
            if !first_time {
                larger = &v[i + 1..v_len - 1];
                smaller = &v[0..i + 1];
            }
            // println!("smaller {:?}, larger {:?}", smaller, larger);
            if smaller.len() > 1 {
                smaller_vec.extend_from_slice(smaller);
                smaller_vec = quicksort(&mut smaller_vec);
                smaller = &smaller_vec[..];
            }
            if larger.len() > 1 {
                larger_vec.extend_from_slice(larger);
                larger_vec = quicksort(&mut larger_vec);
                larger = &larger_vec[..];
            }
            sorted.extend_from_slice(smaller);
            sorted.push(pivot);
            sorted.extend_from_slice(larger);
            // print_vector(&sorted);
            // println!("{}", sorted == v);
        }
    }
    return sorted;
}

fn average(v: &Vec<u32>) -> f32 {
    let mut total: u64 = 0;
    for i in v {
        total += *i as u64;
    }
    let length = v.len() as f32;
    let avg: f32 = total as f32 / length;
    println!("The average is {}", avg);
    avg
}

fn mode(v: &Vec<u32>) -> (u32, u32) {
    let mut map = HashMap::new();
    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    // println!("{:?}", map);

    let mut max = (0, 0);
    for (key, value) in &map {
        let current = map.get(key);
        if current.unwrap() > &max.1 {
            max = (**key, *value)
        }
    }
    let (mode, occurences) = max;
    println!(
        "The mode is {:?} and it's occuring {:?} times.",
        mode, occurences
    );
    max
}

fn create_vector(size: u32, rng_min: u32, rng_max: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for _ in 0..size {
        let secret_number = rand::thread_rng().gen_range(rng_min, rng_max);
        v.push(secret_number);
    }

    v
}

// fn print_vector(v: &Vec<u32>) {
//     for i in v {
//         print!("{}, ", i);
//     }
//     print!("\n");
// }
