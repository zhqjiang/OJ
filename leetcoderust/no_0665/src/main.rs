impl Solution {
    fn is_nums_non_decreasing(nums: &Vec<i32>) -> bool {
        let len = nums.len();
        if len == 0 || len == 1 {
            return true;
        }
        for idx in 0..len - 1 {
            if nums[idx] > nums[idx + 1] {
                return false;
            }
        }

        true
    }

    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let len = nums.len();
        if Solution::is_nums_non_decreasing(&nums) {
            return true;
        }

        let mut index = 0;
        for idx in 0..len - 1 {
            if nums[idx] > nums[idx + 1] {
                index = idx;
                break;
            }
        }

        let mut cloned_nums1 = nums.clone();
        let mut cloned_nums2 = nums.clone();

        cloned_nums1[index] = cloned_nums1[index + 1];
        cloned_nums2[index + 1] = cloned_nums2[index];

        Solution::is_nums_non_decreasing(&cloned_nums1)
            || Solution::is_nums_non_decreasing(&cloned_nums2)
    }
}

struct Solution {}
fn main() {
    // assert_eq!(Solution::check_possibility(vec![9, 3, 5]), true);
    // assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
    assert_eq!(Solution::check_possibility(vec![5, 7, 1, 8]), true);
}
