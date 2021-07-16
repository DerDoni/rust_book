use std::collections::HashMap;

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mean = mean(&vec);
    let median = median(&mut vec);
    println!("Vector: {:?}", &vec);
    println!("Mean: {}", &mean);
    println!("Median: {}", &median);
}

fn mean(ints: &Vec<i32>) -> i32 {
    let mut mean: i32 = 0;

    let length = ints.len();
    for num in ints {
        mean += num;
    }
    mean / length as i32
}

fn median(ints: &mut Vec<i32>) -> i32 {
    ints.sort();

    let length = ints.len();

    ints[length / 2]
}

fn mode(ints: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for number in ints {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
}
