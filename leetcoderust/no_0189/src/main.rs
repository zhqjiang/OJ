fn flip(nums: &mut Vec<i32>, i: usize, j: usize) {
    let mut i = i;
    let mut j = j;
    while i < j {
        j -= 1;
        let tmp = nums[i];
        nums[i] = nums[j];
        nums[j] = tmp;
        i += 1;
    }
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        if k == 0 {
            return;
        }

        flip(nums, 0, n - k);
        flip(nums, n - k, n);
        flip(nums, 0, n);
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
