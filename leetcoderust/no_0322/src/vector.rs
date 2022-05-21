impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut cloned_coins = coins.clone();
        cloned_coins.sort();
        let mut records: Vec<i32> = vec![-1; (amount + 1) as usize];
        records[0] = 0;

        for i in 1..amount + 1 {
            for denomination in cloned_coins.iter() {
                let previous_possible_idx = i - denomination;
                if previous_possible_idx < 0 {
                    break;
                }

                match records.get(previous_possible_idx as usize) {
                    Some(-1) => {
                        // do nothing
                    }
                    Some(value) => {
                        if records[i as usize] == -1 {
                            records[i as usize] = value + 1;
                        } else if value + 1 < records[i as usize] {
                            records[i as usize] = value + 1;
                        }
                    }
                    None => {
                        // do nothing
                    }
                }
            }
        }

        records[amount as usize]
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
