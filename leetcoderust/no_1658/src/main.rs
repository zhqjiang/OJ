use std::collections::HashMap;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        -1
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
}
