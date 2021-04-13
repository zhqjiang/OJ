use std::io::{self, BufRead};

const N: usize = 80;
struct Graph {
    matrices: [[bool; N]; N],
    lists: Vec<Vec<usize>>,
    size: usize,
}

fn go(i: usize, k: i32, used: &mut Vec<bool>, graph: &Graph) -> bool {
    let n = graph.size;
    if k <= 0 {
        return true;
    }
    if i >= n {
        return false;
    }
    if used[i] {
        return go(i + 1, k, used, graph);
    }

    let mut v = Vec::new();
    used[i] = true;

    // push all unused neighbors of location i to v
    for &item in &graph.lists[i] {
        if !(used[item]) {
            v.push(item);
            used[item] = true;
        }
    }

    if go(i + 1, k - 1, used, graph) {
        return true;
    }

    for &item in &v {
        used[item] = false;
    }

    // now deal with neighbors of locaiton i
    // use double for to generate pairs (v[a], v[b])
    let mut v2 = Vec::new();
    for a in 0..v.len() {
        for b in 0..a {
            if !graph.matrices[v[a]][v[b]] {
                used[v[a]] = true;
                used[v[b]] = true;

                for &item in &graph.lists[v[a]] {
                    if !(used[item]) {
                        v2.push(item);
                    };
                }

                for &item in &graph.lists[v[b]] {
                    if !(used[item]) {
                        v2.push(item);
                    }
                }

                for &item in &v2 {
                    used[item] = true;
                }

                if go(i + 1, k - 2, used, graph) {
                    return true;
                }

                for &item in &v2 {
                    used[item] = false;
                }

                v2.clear();
                used[v[a]] = false;
                used[v[b]] = false;
            }
        }
    }
    used[i] = false;
    false
}

fn run(graph: &Graph, k: usize) {
    let mut used = vec![false; graph.size];

    if go(0, k as i32, &mut used, graph) {
        println!("possible");
    } else {
        println!("impossible");
    }
}

fn main() {
    let mut buffer1 = String::new();
    io::stdin().read_line(&mut buffer1).expect("Failed");
    let k = buffer1.trim().parse::<usize>().unwrap();

    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer2).expect("Failed");
    let n = buffer2.trim().parse::<usize>().unwrap();

    if n >= (5 * k - 4) {
        for _ in io::stdin().lock().lines().map(|l| l.unwrap()) {}
        println!("possible");
    } else {
        let mut graph = Graph {
            matrices: [[false; N]; N],
            lists: vec![Vec::new(); n],
            size: n,
        };

        for i in 0..n {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            let nums = buffer
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap() - 1)
                .collect::<Vec<i32>>();

            for j in 1..nums.len() {
                graph.matrices[i][nums[j] as usize] = true;
                graph.lists[i].push(nums[j] as usize);
            }
        }
        run(&graph, k);
    }
}
