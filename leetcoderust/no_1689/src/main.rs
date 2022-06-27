impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let res = n.chars().fold(0, |acc, cur| {
            let num = cur as u8 - '0' as u8;
            if num > acc {
                return num;
            } else {
                return acc;
            }
        });

        res as i32
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::min_partitions(String::from("32")), 3);
    assert_eq!(Solution::min_partitions(String::from("82734")), 8);
    assert_eq!(
        Solution::min_partitions(String::from("27346209830709182346")),
        9
    );
}
