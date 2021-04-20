/*
 * https://open.kattis.com/problems/downtime
 */
use std::cmp::max;
use std::io;

fn sovle(requests: &Vec<i32>, k: usize, n: usize) {
    let mut need_max = 1;

    for i in 0..n {
        let timestamp = requests[i];

        let last_occur_idx = match requests.binary_search(&(timestamp - 1000)) {
            Ok(x) => x as i32,
            Err(x) => x as i32 - 1,
        };

        let need = (i as i32 - last_occur_idx) as usize;

        need_max = max(need_max, need);
    }

    let server_num = if need_max % k == 0 {
        need_max / k
    } else {
        need_max / k + 1
    };

    println!("{}", server_num);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let nums: Vec<usize> = buffer
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let n: usize = nums[0];
    let k: usize = nums[1];

    let mut requests = vec![];

    for _ in 0..n {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let count = buffer.trim().parse::<i32>().unwrap();
        requests.push(count);
    }

    sovle(&requests, k, n);
}
