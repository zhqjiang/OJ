#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // solution 1
        // let mut n = nums.clone();
        // n.sort_by(|x, y| y.cmp(x));
        // n[k as usize - 1]

        // solution 2
        // Runtime and Memory Usage: both 100%
        let mut n: Vec<i32> = Vec::with_capacity(k as usize);
        for item in &nums {
            let idx = n
                .binary_search_by(|probe| item.cmp(probe))
                .unwrap_or_else(|x| x);
            n.insert(idx, *item);
            if n.len() > k as usize {
                n.pop();
            }
        }

        *n.last().unwrap()
    }
}

struct Solution {}

fn main() {
    println!("Hello, world!");
}
