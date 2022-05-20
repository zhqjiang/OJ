impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut record: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];

        for i in 1..m + 1 {
            for j in 1..n + 1 {
                if obstacle_grid[i - 1][j - 1] == 0 {
                    if i == 1 && j == 1 {
                        record[1][1] = 1;
                    } else {
                        record[i][j] = record[i - 1][j] + record[i][j - 1];
                    }
                }
            }
        }

        record[m][n]
    }
}
struct Solution {}
fn main() {
    assert_eq!(
        Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
}
