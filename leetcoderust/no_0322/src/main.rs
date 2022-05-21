use std::collections::HashMap;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut cloned_coins = coins.clone();
        cloned_coins.sort();

        let mut res = HashMap::<i32, i32>::new();
        res.insert(0, 0);

        for i in 1..amount + 1 {
            for denomination in cloned_coins.iter() {
                let previous_possible_idx = i - denomination;

                match res.get(&previous_possible_idx) {
                    Some(v) => match res.get(&i) {
                        Some(v2) => {
                            if *v + 1 < *v2 {
                                res.insert(i, *v + 1);
                            }
                        }
                        None => {
                            res.insert(i, *v + 1);
                        }
                    },
                    None => {}
                }
            }
        }

        match res.get(&amount) {
            Some(v) => return *v,
            None => return -1,
        }
    }
}

struct Solution {}
fn main() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![1, 3], 4), 2);
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 17), 4);
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 1], 21), 3);
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 1], 27), 4);
    assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
}
