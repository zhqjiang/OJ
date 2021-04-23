use std::collections::HashMap;
use std::cmp::max;

fn gen_gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gen_gcd(y, x % y)
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 {
            return n as i32;
        }

        let mut res = 0;

        for i in 0..n {
            let mut map: HashMap<(i32, i32), usize> = HashMap::new();

            for j in i + 1..n {
                let mut x_diff = points[i][0] - points[j][0];
                let mut y_diff = points[i][1] - points[j][1];

                if x_diff == 0 {
                    y_diff = 1;
                } else if y_diff == 0 {
                    x_diff = 1;
                } else {
                    let gcd = gen_gcd(x_diff, y_diff);
                    if gcd != 0 {
                        x_diff /= gcd;
                        y_diff /= gcd;
                    }
                    if x_diff < 0 {
                        x_diff = -x_diff;
                        y_diff = -y_diff;
                    }
                }

                let count = map.entry((x_diff, y_diff)).or_insert(0);
                *count += 1;

                for (_, &count) in &map {
                    res = max(res, count);
                }
            }
        }

        (res + 1) as i32
    }
}
struct Solution {}
fn main() {
    let points1 = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    let points2 = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    assert_eq!(Solution::max_points(points1), 3);
    assert_eq!(Solution::max_points(points2), 4);
}
