struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut new_left = new_interval[0];
        let mut new_right = new_interval[1];

        let mut all_split_points: Vec<i32> = vec![];

        // insert two points to all_split_points
        // or merge a interval with new_interval
        for ele in intervals {
            let ele_left = ele[0];
            let ele_right = ele[1];

            if ele_left > new_right || ele_right < new_left {
                // no intersection, push two points to all_split_points
                all_split_points.push(ele_left);
                all_split_points.push(ele_right);
            } else {
                // merge two intervals
                new_left = if new_left < ele_left {
                    new_left
                } else {
                    ele_left
                };
                new_right = if new_right > ele_right {
                    new_right
                } else {
                    ele_right
                };
            }
        }

        match all_split_points.binary_search(&new_left) {
            Err(e) => {
                all_split_points.insert(e, new_left);
            }
            _ => {}
        }
        match all_split_points.binary_search(&new_right) {
            Err(e) => {
                all_split_points.insert(e, new_right);
            }
            Ok(v) => {
                all_split_points.insert(v, new_right);
            }
        }
        let mut result: Vec<Vec<i32>> = vec![];

        for index in 0..(all_split_points.len() / 2) {
            let a = vec![all_split_points[index * 2], all_split_points[index * 2 + 1]];
            result.push(a);
        }

        result
    }
}

fn main() {
    // let intervals = vec![
    //     vec![1, 2],
    //     vec![3, 5],
    //     vec![6, 6],
    //     vec![8, 10],
    //     vec![12, 16],
    // ];
    // let new_interval = vec![9, 9];
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    let test1 = Solution::insert(intervals, new_interval);
    println!("{:?}", &test1);
}
