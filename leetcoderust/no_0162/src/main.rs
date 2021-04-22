/* logarithmic */
fn is_peak(i: usize, nums: &Vec<i32>) -> bool {
    let n = nums.len();
    if i == 0 {
        if nums[0] > nums[1] {
            return true;
        } else {
            return false;
        }
    } else if i == n - 1 {
        if nums[n - 1] > nums[n - 2] {
            return true;
        } else {
            return false;
        }
    } else {
        if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
            return true;
        } else {
            return false;
        }
    }
}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        let mut i = 0;
        let mut j = n - 1;

        loop {
            if is_peak(i, &nums) {
                return i as i32;
            }
            if is_peak(j, &nums) {
                return j as i32;
            }
            // now mid is not equal to i or j
            let mid = (i + j + 1) / 2;
            if is_peak(mid, &nums) {
                return mid as i32;
            }
            if nums[mid] > nums[mid - 1] {
                i = mid;
            } else if nums[mid] > nums[mid + 1] {
                j = mid;
            } else {
                i = mid; // or j = mid
            }
        }
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
}
