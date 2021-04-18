/*
* https://open.kattis.com/problems/countcircuits
* meet in the middle
*/
use std::io;

struct Bitmask(u32);
impl<'a> IntoIterator for &'a Bitmask {
    type Item = usize;
    type IntoIter = BitmaskIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        BitmaskIntoIter(self, 0)
    }
}
struct BitmaskIntoIter<'a>(&'a Bitmask, usize);
impl<'a> Iterator for BitmaskIntoIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < 32 {
            if ((1 << self.1) & self.0 .0) > 0 {
                self.1 += 1;
                return Some(self.1 - 1);
            } else {
                self.1 += 1;
                return self.next();
            }
        } else {
            None
        }
    }
}

// no point will be on this line(x + 11 * y = 0)
fn offset_to_line(point: &(i16, i16)) -> i16 {
    point.0 + point.1 * 11
}

// brute method to find non-empty subsets
// use bitmask to represent a combination
fn brute(set: &[(i16, i16)], tarsub_of: (i16, i16)) -> u64 {
    let mut count = 0;
    let combinations_up_limit = 1 << set.len();

    for i in 1..combinations_up_limit {
        let sum = Bitmask(i)
            .into_iter()
            .fold((0, 0), |acc, x| (acc.0 + set[x].0, acc.1 + set[x].1));

        if sum == tarsub_of {
            count += 1;
        }
    }

    count
}

fn sub_of(set: &Vec<(i16, i16)>, from: usize, to: usize) -> Vec<(i16, i16)> {
    (set[from..to]).iter().map(|&x| x).collect()
}

fn solve(main: &Vec<(i16, i16)>, other: &Vec<(i16, i16)>) -> u64 {
    let mut count = 0;

    let mut main_occurs: [[u64; 401]; 401] = [[0; 401]; 401];
    let main_up_limit: u32 = 1 << main.len();
    for i in 1..main_up_limit {
        let sum = Bitmask(i)
            .into_iter()
            .fold((0, 0), |acc, x| (acc.0 + main[x].0, acc.1 + main[x].1));

        if sum == (0, 0) {
            count += 1;
        } else {
            main_occurs[(sum.0 + 200) as usize][(sum.1 + 200) as usize] += 1;
        }
    }

    let mut other_occurs: [[u64; 401]; 401] = [[0; 401]; 401];
    let other_up_limit: u32 = 1 << other.len();
    for i in 1..other_up_limit {
        let sum = Bitmask(i)
            .into_iter()
            .fold((0, 0), |acc, x| (acc.0 + other[x].0, acc.1 + other[x].1));

        other_occurs[(sum.0 + 200) as usize][(sum.1 + 200) as usize] += 1;
    }

    for i in 0..401 {
        for j in 0..401 {
            count += main_occurs[i][j] * other_occurs[400 - i][400 - j];
        }
    }

    count
}

fn split_set(set: &Vec<(i16, i16)>) -> u64 {
    let mut set = set.clone();
    set.sort_by(|a, b| offset_to_line(a).cmp(&offset_to_line(b)));

    let split = (set.len() + 1) / 2;
    let split_offset = offset_to_line(&set[split]);

    if split_offset > 0 {
        let main = sub_of(&set, 0, split);
        let other = sub_of(&set, split, set.len());
        solve(&main, &other)
    } else {
        let other = sub_of(&set, 0, split);
        let main = sub_of(&set, split, set.len());
        solve(&main, &other)
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut a = vec![];
    for _ in 0..n {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let nums: Vec<i16> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        a.push((nums[0], nums[1]));
    }

    let res = if a.len() > 4 {
        split_set(&a)
    } else {
        brute(&a, (0, 0))
    };

    println!("{}", res);
}
