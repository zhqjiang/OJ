/*
 * https://open.kattis.com/problems/knapsack
 */
use std::cmp::max;
use std::io;

fn cal_items(
    i: usize,
    cap: usize,
    items: &mut Vec<usize>,
    best: &Vec<Vec<usize>>,
    vs: &Vec<usize>,
    ws: &Vec<usize>,
) {
    if i == 0 {
        return;
    }
    if best[i][cap] > best[i - 1][cap] {
        items.push(i - 1);
        cal_items(i - 1, cap - ws[i - 1], items, best, vs, ws)
    } else {
        cal_items(i - 1, cap, items, best, vs, ws);
    }
}

fn solve(cap: usize, n: usize, vs: &Vec<usize>, ws: &Vec<usize>) {
    let mut best = vec![vec![0; cap + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=cap {
            if i == 0 || j == 0 {
                best[i][j] = 0;
            } else if j >= ws[i - 1] {
                best[i][j] = max(best[i - 1][j], best[i - 1][(j - ws[i - 1])] + vs[i - 1]);
            } else {
                best[i][j] = best[i - 1][j];
            }
        }
    }
    let mut items = vec![];
    cal_items(n, cap, &mut items, &best, vs, ws);

    println!("{}", items.len());
    let output = items
        .iter()
        .rev()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let nums: Vec<usize> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if nums.len() < 2 {
            break;
        }
        let c = nums[0];
        let n = nums[1];

        let mut values = vec![];
        let mut weights = vec![];
        for _ in 0..n {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            let nums: Vec<usize> = buffer
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let v = nums[0];
            let w = nums[1];
            values.push(v);
            weights.push(w);
        }

        solve(c, n, &values, &weights);
    }
}
