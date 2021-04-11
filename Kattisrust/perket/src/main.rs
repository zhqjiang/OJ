use std::io::{self, BufRead};

fn run(v: &Vec<(i32, i32)>) -> i32 {
    let mut min = (v[0].0 - v[0].1).abs();

    let mut sums: Vec<(i32, i32)> = Vec::new();
    for &i in v {
        let mut to_be_addded: Vec<(i32, i32)> = Vec::new();
        for &j in &sums {
            let new_item = (i.0 * j.0, i.1 + j.1);

            let diff = (new_item.0 - new_item.1).abs();
            if diff == 0 {
                return 0;
            }
            if diff < min {
                min = diff
            }

            to_be_addded.push(new_item);
        }

        for &j in &to_be_addded {
            sums.push(j);
        }
        sums.push(i);
        let diff = (i.0 - i.1).abs();
        if diff < min {
            min = diff
        }
    }
    min
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let counts = buffer.trim().parse::<usize>().unwrap();

    let mut v: Vec<(i32, i32)> = Vec::with_capacity(counts);
    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        v.push((nums[0], nums[1]));
    }

    println!("{}", run(&v));
}
