/*
* Geppetto
* https://open.kattis.com/problems/geppetto
* Run time: 0.00s
*/
use std::io;

fn solve(m: &[u32; 20], n: usize) -> usize {
    let max = <u32>::pow(2, n as u32);
    let mut recording: Vec<bool> = vec![false; max as usize];
    recording[0] = true;
    let mut count = 1; // this 1 represents no ingredients

    for i in 0..n {
        let i = i as usize;
        let left = <u32>::pow(2, i as u32) as usize;
        let right = <u32>::pow(2, i as u32 + 1) as usize;
        for c in left..right {
            // c represents combination
            // for example
            // c = 7(0b1101) represents ingredients 0, ingredients 2 and ingredients 3
            if !recording[(c - left)] {
                recording[c] = false;
            } else {
                if (m[i] as usize & (c - left)) == 0 {
                    // if c is 0b1101
                    // then c - left is 0b0101
                    // use bit operation
                    // to check whether ingredient i is conflicted with other ingredients in (c-left)
                    recording[c] = true;
                    count += 1;
                } else {
                    recording[c] = false;
                }
            }
        }
    }

    count
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let nums: Vec<usize> = buffer
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let n = nums[0];
    let m = nums[1];
    let mut matrices: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut row_bits: [u32; 20] = [0; 20];

    for _ in 0..m {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let nums: Vec<usize> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if matrices[nums[0] - 1][nums[1] - 1] == false {
            matrices[nums[0] - 1][nums[1] - 1] = true;
            matrices[nums[1] - 1][nums[0] - 1] = true;
            row_bits[nums[0] - 1] += 1 << (nums[1] - 1);
            row_bits[nums[1] - 1] += 1 << (nums[0] - 1);
        }
    }
    if n == 1 {
        println!("2");
    } else {
        println!("{}", solve(&row_bits, n));
    }
}
