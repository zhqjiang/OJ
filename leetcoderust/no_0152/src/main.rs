/*
* divide elements by zero elements
* then calculate max value of every non-zero part
*
 */
use std::cmp::max;

fn cal(left: usize, right: usize, nums: &Vec<i32>) -> i32 {
    let n = nums.len();
    if left >= right || left >= n || right > n {
        return nums[0];
    }
    if left + 1 == right {
        return nums[left];
    }

    let mut negatives_counts = 0;
    let mut negatives = vec![];

    for i in left..right {
        if nums[i] < 0 {
            negatives_counts += 1;
            if negatives.len() == 0 {
                negatives.push(i);
                negatives.push(i);
            } else {
                negatives[1] = i;
            }
        }
    }

    if negatives_counts % 2 == 0 {
        return nums[left..right].iter().fold(1, |acc, &x| acc * x);
    } else {
        return max(
            nums[(negatives[0] + 1)..right]
                .iter()
                .fold(1, |acc, &x| acc * x),
            nums[left..negatives[1]].iter().fold(1, |acc, &x| acc * x),
        );
    }
}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let n = nums.len();
        let mut res = nums[0];

        let mut there_is_zero = false;
        let mut left = 0;
        let mut right = 0;

        for i in 0..n {
            if nums[i] == 0 {
                there_is_zero = true;
                res = max(res, cal(left, right, &nums));
                left = i + 1;
            } else {
                right = i + 1;
            }
        }
        res = max(res, cal(left, right, &nums));
        if there_is_zero {
            res = max(res, 0);
        }

        res
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::max_product(vec![3, -2, 4]), 4);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![-5, -2, 0, -1]), 10);
    assert_eq!(Solution::max_product(vec![10, 5, 2, 0, -1, 35]), 100);
    assert_eq!(Solution::max_product(vec![0, 0, 10, -5, 2, 0, -1, 5]), 10);
    assert_eq!(Solution::max_product(vec![0, 0, -10, 0, 2, 0, -1, 0]), 2);
    assert_eq!(Solution::max_product(vec![0, 0, -10, 0, -2, 0, -1, 0]), 0);
}
