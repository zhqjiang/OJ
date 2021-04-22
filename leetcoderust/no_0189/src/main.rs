use std::cmp::max;
use std::cmp::min;

fn flip(nums: &mut Vec<i32>, i: usize, j: usize) {
    let mut i = min(i, j);
    let mut j = max(i, j);
    while i < j {
        let tmp = nums[j];
        nums[j] = nums[i];
        nums[i] = tmp;
        i += 1;
        j -= 1;
    }
}

fn change(nums: &mut Vec<i32>, i: usize, j: usize) {
    let mut m = 0;
    let mut n = j;

    loop {
        if n == nums.len() {
            break;
        }
        let tmp = nums[n];
        nums[n] = nums[m];
        nums[m] = tmp;
        m += 1;
        n += 1;
    }
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        if k == 0 {
            return;
        }

        if k <= n / 2 {
            flip(nums, 0, n - k - 1);
            change(nums, k - 1, n - k);
            flip(nums, k, n - k - 1);
            flip(nums, n - k, n - 1);
        } else {
            flip(nums, n - k, n - 1);
            flip(nums, n - k, k - 1);
            change(nums, n - k - 1, k);
            flip(nums, 0, n - k - 1);
        }
    }
}
struct Solution {}
fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut a, 5);
    println!("{:?}", a);

    let mut b = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut b, 3);
    println!("{:?}", b);
}
