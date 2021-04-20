#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut n = nums.clone();
        n.sort();
        n.dedup();
        if n.len() < nums.len() {
            return true;
        } else {
            return false;
        }
    }
}
struct Solution {}

fn main() {}
