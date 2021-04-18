// https://open.kattis.com/problems/maxloot not finished
use std::cmp::max;
use std::io;

struct Items {
    values: Vec<u32>,
    costs: Vec<u32>,
    size: usize,
}

fn run(at: usize, c_sum: u32, cap: u32, items: &Items) -> u32 {
    let Items {
        values,
        costs,
        size,
    } = items;

    if at == *size {
        return 0;
    }

    let v = values[at];
    let cost = costs[at];

    if (cost + c_sum) > cap {
        return 0;
    }

    // let remain =

    max(
        v + run(at + 1, c_sum + cost, cap, items),
        run(at + 1, c_sum, cap, items),
    )
}

fn solve(cap: u32, items: &Items) -> u32 {
    run(0, 0, cap, items)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let testcase_count = buffer.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let nums: Vec<u32> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let n = nums[0] as usize;
        let cap = nums[1];

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let values: Vec<u32> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let costs: Vec<u32> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let items = Items {
            values,
            costs,
            size: n,
        };
        println!("{}", solve(cap, &items));
    }
}
/*
fn solve(n: usize, cap: u32, values: &Vec<u32>, costs: &Vec<u32>) -> u32 {
    let whole_value = values.iter().fold(0, |acc, &x| acc + x);
    let whole_costs = costs.iter().fold(0, |acc, &x| acc + x);

    if whole_costs <= cap {
        return whole_value;
    }

    let w = whole_costs - cap;

    // choose k items, make costs of them not less than w, but has minimal values

    // let k = (whole_value as f32).log2().round() as u32;
    // let max_n + 2 exp
    // 23
// }
*/
