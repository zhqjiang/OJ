impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let all_sum = card_points.iter().fold(0, |a, b| a + b);
        if k >= card_points.len() as i32 {
            return all_sum;
        }

        let t = (card_points.len() as i32 - k) as usize;
        let mut sum = card_points[0..t].iter().fold(0, |a, b| a + b);
        let mut min_sum = sum;
        let mut left = 0 as usize;
        let mut right = t as usize;

        while right < card_points.len() {
            sum = sum - card_points[left] + card_points[right];
            min_sum = if sum < min_sum { sum } else { min_sum };
            left += 1;
            right += 1;
        }

        all_sum - min_sum
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    assert_eq!(
        Solution::max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3),
        202
    );
}
