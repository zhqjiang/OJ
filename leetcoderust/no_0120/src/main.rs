impl Solution {
    pub fn min(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let height = triangle.len();
        let mut rec = vec![0; height + 1];
        for i in (0..height).rev() {
            rec = triangle[i]
                .iter()
                .enumerate()
                .map(|(idx, &ele)| ele + Solution::min(rec[idx], rec[idx + 1]))
                .collect();
        }

        rec[0]
    }
}
struct Solution {}
fn main() {
    assert_eq!(
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
}
