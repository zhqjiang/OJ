/*
* method: depth first search
*
 */

use std::collections::HashMap;
type Adjacent = Vec<HashMap<usize, bool>>;

fn dfs(
    at: usize,
    s: &mut Vec<usize>,
    used: &mut Vec<bool>,
    visited: &mut Vec<bool>,
    adj: &Adjacent,
) {
    let n = adj.len();
    if s.len() >= n || used[at] {
        return;
    }
    if !used[at] && visited[at] {
        // there is a circuit
        *s = vec![0; n + 1];
        return;
    }

    visited[at] = true;
    if adj[at].keys().len() == 0 {
        s.push(at);
        used[at] = true;
    } else {
        for (&i, _) in &adj[at] {
            dfs(i, s, used, visited, adj);
        }
        if s.len() < n {
            s.push(at);
            used[at] = true;
        }
    }
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        if n == 1 {
            return vec![0];
        }

        let mut adj: Adjacent = vec![HashMap::new(); num_courses as usize];
        for p in prerequisites {
            let idx1 = p[0] as usize;
            let idx2 = p[1] as usize;
            adj[idx2].insert(idx1, true);
        }

        let mut visited = vec![false; n];
        let mut used = vec![false; n];
        let mut s = vec![];

        for i in 0..n {
            dfs(i, &mut s, &mut used, &mut visited, &adj);
        }

        if s.len() > n {
            vec![]
        } else {
            s.iter().rev().map(|&x| x as i32).collect()
        }
    }
}

struct Solution {}
fn main() {
    // test might fail, because there may be mutliple orderings
    assert_eq!(Solution::find_order(1, vec![]), vec![0]);
    assert_eq!(
        Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2],]),
        vec![0, 2, 1, 3]
    );
    assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    assert_eq!(
        Solution::find_order(2, vec![vec![1, 0], vec![0, 1]]),
        vec![]
    );
    assert_eq!(
        Solution::find_order(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
        vec![2, 1, 0]
    );
}
