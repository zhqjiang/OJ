use std::collections::HashMap;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let length = nums.len();
        let mut left_sums = vec![0; length];

        let mut res = -1;

        let mut left_record: HashMap<i32, i32> = HashMap::new();
        for (idx, num) in nums.clone().into_iter().enumerate() {
            left_sums[idx] = if idx > 0 { left_sums[idx - 1] } else { 0 } + num;

            if left_sums[idx] <= x {
                left_record.insert(x - left_sums[idx], (idx + 1) as i32);
                if left_sums[idx] == x {
                    res = (idx + 1) as i32;
                }
            }
        }

        left_record.insert(x, 0);

        let mut right_sums = vec![0; length];
        for (idx, num) in nums.into_iter().rev().enumerate() {
            right_sums[idx] = if idx > 0 { right_sums[idx - 1] } else { 0 } + num;

            if right_sums[idx] <= x {
                let right_count = (idx + 1) as i32;
                match left_record.get(&right_sums[idx]) {
                    Some(left_count) => {
                        let this_count = left_count + right_count;

                        if this_count <= (length as i32) {
                            if res == -1 || res > this_count {
                                res = this_count;
                            }
                        }
                    }
                    None => {}
                }
            }
        }

        res
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
}
