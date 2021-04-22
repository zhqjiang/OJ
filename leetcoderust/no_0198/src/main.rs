use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }

        let mut best_one = vec![0; n];
        let mut best_two = vec![0; n];

        best_one[0] = nums[0];
        best_two[0] = 0;

        for i in 1..n {
            best_one[i] = best_two[i - 1] + nums[i];
            best_two[i] = max(best_one[i - 1], best_two[i - 1]);
        }

        max(best_one[n - 1], best_two[n - 1])
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}
