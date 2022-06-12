use std::collections::HashSet;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut i = 0;
        let mut j = 0;
        let mut set = HashSet::new();

        let mut sum = 0;
        let mut res = 0;

        while i < length && j < length {
            if !set.contains(&nums[j]) {
                sum += nums[j];
                res = if sum > res { sum } else { res };
                set.insert(nums[j]);
                j = j + 1;
            } else {
                sum = sum - nums[i];
                set.remove(&nums[i]);
                i = i + 1;
            }
        }

        res
    }
}

struct Solution {}
fn main() {
    assert_eq!(Solution::maximum_unique_subarray(vec![1, 7]), 8);
    assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
    assert_eq!(
        Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]),
        8
    );
}
