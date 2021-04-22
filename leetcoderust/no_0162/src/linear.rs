impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        for i in 0..n {
            if i == 0 {
                if nums[0] > nums[1] {
                    return 0 as i32;
                }
            } else if i == n - 1 {
                if nums[n - 1] > nums[n - 2] {
                    return (n - 1) as i32;
                }
            } else {
                if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
                    return i as i32;
                }
            }
        }

        (n - 1) as i32 // useless
    }
}
struct Solution {}
