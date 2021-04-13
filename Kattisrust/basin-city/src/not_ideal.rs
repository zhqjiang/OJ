use std::io;

// convert number to binary format, calculate the counts of 1
// example: 380 = 0b0101111100u128 -> 6
fn length(n1: u128) -> usize {
    let mut k = n1;
    let mut res: usize = 0;
    while k > 0 {
        res += (k & 1) as usize;
        k = k >> 1;
    }
    res
}

fn indexs(n1: u128) -> Vec<usize> {
    let mut res = Vec::new();

    let mut i = 0;
    while i < 71 {
        if ((1 << i) & n1) > 0 {
            res.push(i);
        }
        i += 1;
    }
    res
}

fn lowest(n1: u128) -> usize {
    let mut res = 127;
    for i in 0..71 {
        if ((1 << i) & n1) > 0 {
            res = i;
            break;
        }
    }
    res
}

fn run(at: usize, placed: u128, traversed: u128, n: usize, k: usize, adj: &Vec<u128>) -> bool {
    let not_traversed_locations = n - length(traversed);
    let not_placed_drones = k - length(placed);
    if not_traversed_locations < not_placed_drones {
        return false;
    }
    if not_traversed_locations == 0 && not_placed_drones > 0 {
        return false;
    }
    if not_placed_drones == 0 {
        // println!("{:b}", placed);
        return true;
    }

    let mut res;

    // put a drone at location at
    let new_traversed = traversed | adj[at] | (1 << at);
    let new_at = lowest(((1 << n) - 1) & (!new_traversed));
    res = run(new_at, placed | (1 << at), new_traversed, n, k, adj);

    // location at is empty
    let neighbors = indexs(adj[at] & (!traversed));
    if neighbors.len() == 4 {
        for (n1, n2) in vec![
            (neighbors[0], neighbors[1]),
            (neighbors[0], neighbors[2]),
            (neighbors[0], neighbors[3]),
            (neighbors[1], neighbors[2]),
            (neighbors[1], neighbors[3]),
            (neighbors[2], neighbors[3]),
        ] {
            if adj[n1] & (1 << n2) == 0 {
                let new_traversed =
                    traversed | adj[n1] | adj[n2] | (1 << n1) | (1 << n2) | (1 << at);
                let new_at = lowest(((1 << n) - 1) & (!new_traversed));
                let new_placed = placed | (1 << n1) | (1 << n2);
                res = res || run(new_at, new_placed, new_traversed, n, k, adj);
            }
        }
    } else {
        for neighbor in neighbors {
            let new_traversed = traversed | adj[neighbor] | (1 << neighbor) | (1 << at);
            let new_at = lowest(((1 << n) - 1) & (!new_traversed));
            let new_placed = placed | (1 << neighbor);
            res = res || run(new_at, new_placed, new_traversed, n, k, adj);
        }
    }

    res
}

fn solve(n: usize, k: usize, adj: &Vec<u128>) {
    if run(0, 0, 0, n, k, adj) {
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
        println!("possible");
    } else {
        let mut adj: Vec<u128> = Vec::with_capacity(n);

        for _ in 0..n {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            let mut nums = buffer
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap() - 1)
                .collect::<Vec<i32>>();
            // use bits to represent adjacent list instead of double vector
            let item = nums.drain(1..).fold(0, |acc, x| acc + (1 << x));
            adj.push(item);
        }

        solve(n, k, &adj);
    }
}
