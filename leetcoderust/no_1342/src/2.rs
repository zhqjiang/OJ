impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            return 0;
        } else {
            return if num % 2 == 1 {
                1 + Solution::number_of_steps(num - 1)
            } else {
                1 + Solution::number_of_steps(num / 2)
            }
        }
    }
}