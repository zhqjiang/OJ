fn dfs(
    i: usize,
    j: usize,
    visited: &mut Vec<Vec<bool>>,
    num: &mut i32,
    grid: &Vec<Vec<char>>,
    flag_i: &usize,
    flag_j: &usize,
) {
    if visited[i][j] {
        return;
    }
    visited[i][j] = true;

    if grid[i][j] == '0' {
        return;
    }
    let m = grid.len();
    let n = grid[0].len();

    if i > 0 {
        dfs(i - 1, j, visited, num, grid, flag_i, flag_j);
    }
    if i < m - 1 {
        dfs(i + 1, j, visited, num, grid, flag_i, flag_j);
    }
    if j > 0 {
        dfs(i, j - 1, visited, num, grid, flag_i, flag_j);
    }
    if j < n - 1 {
        dfs(i, j + 1, visited, num, grid, flag_i, flag_j);
    }
    if i == *flag_i && j == *flag_j && grid[i][j] == '1' {
        *num += 1;
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut visited = vec![vec![false; n]; m];
        let mut num = 0;

        for i in 0..m {
            for j in 0..n {
                dfs(i, j, &mut visited, &mut num, &grid, &i, &j);
            }
        }

        num
    }
}
struct Solution {}
fn main() {
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0',],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );

    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0',],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '1', '0', '0']
        ]),
        3
    );
}
