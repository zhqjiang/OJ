/*
* https://open.kattis.com/problems/closestsums
* meet in the middle
*/
use std::io;

struct Case {
    n: usize,
    set: Vec<i32>,
    m: usize,
    tests: Vec<i32>,
}

fn solve(target: &i32, set: &Vec<i32>) -> i32 {
    let mut s = set.clone();
    s.sort();

    // [0, mid) [mid, s.len())
    let mut mid = 0;
    match s.binary_search(&((target + 1) / 2)) {
        Ok(i) => mid = i,
        Err(i) => mid = i,
    }
    if mid == 0 {
        return s[0] + s[1];
    }
    if mid == s.len() {
        return s[mid - 1] + s[mid - 2];
    }
    if s.len() == 2 {
        return s[0] + s[1];
    }

    let (left, right) = s.split_at(mid);
    let mut res = s[mid - 1] + s[mid];
    let mut diff = ((s[mid - 1] + s[mid]) - target).abs();

    for &i in left {
        for &j in right {
            let sum = i + j;
            if (sum - target).abs() < diff {
                res = sum;
                diff = (sum - target).abs();
            }
        }
    }

    res
}

fn main() {
    let mut cases = vec![];
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");

        let mut n = 0;
        match buffer.trim().parse::<usize>() {
            Ok(x) => n = x,
            Err(_) => {
                break;
            }
        };

        let mut set = Vec::with_capacity(n);

        for _ in 0..n {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            let num = buffer.trim().parse::<i32>().unwrap();
            set.push(num);
        }

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let m = buffer.trim().parse::<usize>().unwrap();

        let mut tests = Vec::with_capacity(m);
        for _ in 0..m {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            let num = buffer.trim().parse::<i32>().unwrap();
            tests.push(num);
        }

        cases.push(Case { n, set, m, tests });
    }

    for i in 0..cases.len() {
        println!("Case {}:", i + 1);
        for target in &cases[i].tests {
            let result = solve(target, &cases[i].set);
            println!("Closest sum to {} is {}.", target, result);
        }
    }
}
