impl Solution {
    // pub fn missing_number(nums: Vec<i32>) -> i32 {
    //     for k in 0..=nums.len() as i32 {
    //         if !nums.contains(&k) {
    //             return k;
    //         }
    //     }
    //     return 0;
    // }

    // pub fn missing_number(nums: Vec<i32>) -> i32 {
    //     let mut s = nums.clone();
    //     s.sort();

    //     for (idx, value) in s.iter().enumerate() {
    //         if *value != idx as i32 {
    //             return idx as i32;
    //         }
    //     }

    //     nums.len() as i32
    // }

    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut ret = nums.len() as i32;
        for i in 0..nums.len() {
            ret ^= i as i32 ^ nums[i];
        }
        ret
    }
}

struct Solution {}
fn main() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![0, 1]), 2);
}
