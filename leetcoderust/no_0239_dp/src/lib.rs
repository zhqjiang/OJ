#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
        assert_eq!(
            Solution::max_sliding_window(vec![-7, -8, 7, 5, 7, 1, 6, 0], 4),
            vec![7, 7, 7, 7, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3),
            vec![3, 3, 2, 5]
        );
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        if n == 1 || k == 1 {
            return nums;
        }
        let final_res_size = n - (k as usize) + 1;
        let mut res = vec![-10001; final_res_size];
        let mut order_lists: Vec<(i32, usize)> = Vec::with_capacity(n);

        // insert sort O(n)
        for i in 0..n {
            let value = nums[i];
            order_lists.push((value, i));

            for j in (0..i).rev() {
                if value > order_lists[j].0 {
                    order_lists.swap(j + 1, j);
                } else {
                    break;
                }
            }
        }

        let mut count = 0;
        for i in 0..n {
            if count == final_res_size {
                // if equal, all elements are filled in res
                break;
            }
            let value = order_lists[i].0;
            let index = order_lists[i].1;

            for j in (index as i32 - k + 1)..(index as i32 + 1) {
                if j >= 0 && j < final_res_size as i32 {
                    if res[j as usize] < value {
                        res[j as usize] = value;
                        count += 1;
                    }
                }
            }
        }

        res
    }
}
struct Solution {}
