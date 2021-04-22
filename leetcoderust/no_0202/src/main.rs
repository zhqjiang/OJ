use std::collections::HashMap;

fn gen(n: i32) -> i32 {
    let mut n = n;
    let mut res = 0;
    while n > 0 {
        res += (n % 10) * (n % 10);
        n = n / 10;
    }

    res
}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        let mut rec = HashMap::new();

        let mut r = n;
        loop {
            r = gen(r);
            if r == 1 {
                return true;
            }
            match rec.get(&r) {
                Some(_) => {
                    return false;
                }
                None => {
                    rec.insert(r, true);
                }
            }
        }
    }
}
struct Solution {}

fn main() {
    assert_eq!(Solution::is_happy(19), true);
    // assert_eq!(Solution::is_happy(2), false);
}
