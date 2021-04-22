impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut guess = nums[0];
        let mut count = 0;
        for i in nums {
            if count == 0 {
                guess = i;
                count += 1;
            } else if i == guess {
                count += 1;
            } else {
                count -= 1;
            }
        }
        guess
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
