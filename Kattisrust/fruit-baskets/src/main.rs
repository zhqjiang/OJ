use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let n = buffer.trim().parse::<u32>().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed");
    let nums: Vec<u64> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let s = nums.iter().fold(0, |acc, &x| acc + x);
    let mut whole = u64::pow(2, n - 1) * s;

    // find all fruit baskets which are smaller than 200
    for &i in &nums {
        if i < 200 {
            whole -= i
        }
    }

    for i in 0..n as usize {
        for j in 0..i {
            let weight = &nums[i] + &nums[j];
            if weight < 200 {
                whole -= weight;
            }
        }
    }

    for i in 0..n as usize {
        for j in 0..i {
            for k in 0..j {
                let weight = &nums[i] + &nums[j] + &nums[k];
                if weight < 200 {
                    whole -= weight;
                }
            }
        }
    }

    println!("{}", whole);
}
