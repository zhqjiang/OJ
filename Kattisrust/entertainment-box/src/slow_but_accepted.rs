/*
* https://open.kattis.com/problems/entertainmentbox
* slow
*/
use std::cmp::Ordering;
use std::io;

struct Show {
    begin: u32,
    end: u32,
}

fn compare(a: &Show, b: &Show) -> Ordering {
    if a.end < b.end {
        Ordering::Less
    } else if a.end == b.end {
        if a.begin <= b.begin {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Greater
    }
}

fn solve(shows: &Vec<Show>, k: usize, n: usize) {
    let mut count = 0;
    let mut slots = vec![];

    // deal with first show
    count += 1;
    slots.push(shows[0].end);

    for i in 1..n {
        let show = &shows[i];

        // special case 1
        if show.begin < slots[0] {
            if slots.len() >= k {
                continue;
            } else {
                count += 1;
                slots.push(show.end);
                continue;
            }
        }

        // special case 2
        let slot_size = slots.len();
        if show.begin >= slots[slot_size - 1] {
            count += 1;
            slots[slot_size - 1] = show.end;
            continue;
        }

        // normal situation
        let choosed_slot_idx = match slots.binary_search(&show.begin) {
            Ok(x) => x,
            Err(x) => x - 1,
        };

        for j in choosed_slot_idx..slot_size - 1 {
            slots[j] = slots[j + 1];
        }
        slots[slot_size - 1] = show.end;
        count += 1;
    }

    println!("{}", count);
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

    let mut shows = Vec::with_capacity(n);
    for _ in 0..n {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let nums: Vec<u32> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        shows.push(Show {
            begin: nums[0],
            end: nums[1],
        });
    }

    shows.sort_by(|a, b| compare(a, b));

    solve(&shows, k, n);
}
