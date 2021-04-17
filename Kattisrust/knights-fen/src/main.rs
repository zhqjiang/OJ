/*
* https://open.kattis.com/problems/knightsfen
*/
use std::cmp::min;
use std::io;

const MAX: usize = 10;
const TARGET: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1],
    [0, 0, 9, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0],
];

fn target_reached(m: [[u8; 5]; 5], steps: usize) -> (bool, bool) {
    let mut is_success = true;

    let mut real_dif = 0;
    let possible_change = (MAX + 1 - steps) * 2;

    for i in 0..5 {
        for j in 0..5 {
            if TARGET[i][j] != m[i][j] {
                is_success = false;
                real_dif += 1;
            }
        }
    }

    (is_success, real_dif > possible_change)
}

fn gen_ps(now: (usize, usize), from: (usize, usize)) -> Vec<(i32, i32)> {
    let x = now.0 as i32;
    let y = now.1 as i32;
    let mut all = vec![
        (x + 2, y + 1),
        (x + 2, y - 1),
        (x - 2, y + 1),
        (x - 2, y - 1),
        (x + 1, y + 2),
        (x + 1, y - 2),
        (x - 1, y + 2),
        (x - 1, y - 2),
    ];

    all.retain(|(i, j)| *i >= 0 && *i <= 4 && *j >= 0 && *j <= 4);
    all.retain(|(i, j)| (*i * 10 + *j) != (from.0 as i32 * 10 + from.1 as i32));

    all
}

fn step(now: (usize, usize), new: (usize, usize), m: [[u8; 5]; 5]) -> [[u8; 5]; 5] {
    let mut m = m;

    let tmp = m[new.0][new.1];
    m[new.0][new.1] = 9;
    m[now.0][now.1] = tmp;

    m
}

fn go(steps: usize, from: (usize, usize), now: (usize, usize), m: [[u8; 5]; 5]) -> usize {
    if steps > MAX {
        return MAX + 1;
    }
    let (is_success, not_possible) = target_reached(m, steps);
    if is_success {
        return steps;
    }
    if not_possible {
        return MAX + 1;
    }

    let allowed = gen_ps(now, from);
    let mut t = MAX + 1;

    for new in allowed {
        let new = (new.0 as usize, new.1 as usize);
        let new_m = step(now, new, m);
        t = min(t, go(steps + 1, now, new, new_m));
    }

    return t;
}

fn run(m: [[u8; 5]; 5]) {
    let mut x = 0;
    let mut y = 0;
    for i in 0..5 {
        for j in 0..5 {
            if m[i][j] == 9 {
                x = i;
                y = j;
            }
        }
    }

    let steps = go(0, (9, 9), (x, y), m);
    if steps >= 11 {
        println!("Unsolvable in less than {} move(s).", MAX + 1);
    } else {
        println!("Solvable in {} move(s).", steps);
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let counts = buffer.trim().parse::<i16>().unwrap();
    for _ in 0..counts {
        let mut m: [[u8; 5]; 5] = [[9; 5]; 5];

        for i in 0..5 {
            let mut s1 = String::new();
            io::stdin().read_line(&mut s1).expect("Failed");

            let s2 = s1.trim_end_matches('\n');
            let c: Vec<char> = s2.chars().collect();
            for (j, &item) in c.iter().enumerate() {
                if item == '1' {
                    m[i][j] = 1;
                } else if item == '0' {
                    m[i][j] = 0;
                }
            }
        }

        run(m);
    }
}
