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

use std::cmp::Ordering;
use std::collections::VecDeque;

struct MonotonicQueue<T>
where
    T: Ord + Clone,
{
    data: VecDeque<(T, usize)>, // (number: T, num_elements: usize)
    length: usize,
    window_size: Option<usize>,
}

impl<T> MonotonicQueue<T>
where
    T: Ord + Clone,
{
    fn new() -> MonotonicQueue<T> {
        MonotonicQueue {
            data: VecDeque::new(),
            length: 0,
            window_size: None,
        }
    }

    fn from_capacity_and_window_size(len: usize, size: usize) -> MonotonicQueue<T> {
        MonotonicQueue {
            data: VecDeque::with_capacity(len),
            length: 0,
            window_size: Some(size),
        }
    }

    fn push(&mut self, new_ele: T) {
        let mut num_elements: usize = 0;
        let mut to_be_truncated_count = 0;

        for item in self.data.iter().rev() {
            if item.0.cmp(&new_ele) == Ordering::Less {
                num_elements += item.1 + 1;
                to_be_truncated_count += 1;
            } else {
                break;
            }
        }

        self.data.truncate(self.data.len() - to_be_truncated_count);
        self.data.push_back((new_ele, num_elements));
        self.length += 1;

        match self.window_size {
            Some(x) if self.length > x => {
                self.pop();
            }
            _ => {}
        }
    }

    fn pop(&mut self) {
        match self.data.front_mut() {
            Some(x) => {
                if x.1 > 0 {
                    x.1 -= 1;
                } else {
                    self.data.pop_front();
                }
            }
            _ => {}
        }

        self.length -= 1;
    }

    fn max(&self) -> Option<&T> {
        match self.data.front() {
            Some(x) => Some(&x.0),
            _ => None,
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn set_window_size(&mut self, len: usize) {
        self.window_size = Some(len);
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());

        let mut q: MonotonicQueue<i32> =
            MonotonicQueue::from_capacity_and_window_size(nums.len(), k as usize);

        for item in nums.iter() {
            q.push(*item);
            if q.length == k as usize {
                match q.max() {
                    Some(x) => {
                        res.push(*x);
                    }
                    None => {}
                }
            }
        }

        res
    }
}

struct Solution {}
