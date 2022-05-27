impl Solution {
    // pub fn number_of_steps(num: i32) -> i32 {
    //     if num == 0 {
    //         return 0;
    //     } else {
    //         return if num % 2 == 1 {
    //             1 + Solution::number_of_steps(num - 1)
    //         } else {
    //             1 + Solution::number_of_steps(num / 2)
    //         }
    //     }
    // }
    pub fn number_of_steps(num: i32) -> i32 {
        match (num, num % 2 == 1) {
            (0, _) => 0,
            (_, true) => 1 + Solution::number_of_steps(num - 1),
            (_, false) => 1 + Solution::number_of_steps(num / 2),
        }
    }
}

struct Solution {}
fn main() {
    assert_eq!(Solution::number_of_steps(4), 2);
}
