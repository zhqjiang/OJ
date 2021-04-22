/*
* method: in-degrees
* use adjacent list(with HashMap) instead of matrix may be better
 */
 impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        if n == 1 {
            return vec![0];
        }

        let mut incomings = vec![0; num_courses as usize];
        let mut ma: Vec<Vec<bool>> = vec![vec![false; num_courses as usize]; num_courses as usize];
        for p in prerequisites {
            let idx1 = p[0] as usize;
            let idx2 = p[1] as usize;
            if ma[idx2][idx1] {
                return vec![];
            } else {
                incomings[idx1] += 1;
                ma[idx1][idx2] = true;
            }
        }
        // vertices with no incoming edges
        let mut vs: Vec<usize> = incomings
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 0)
            .map(|(i, _)| i)
            .collect();

        let mut ordering: Vec<usize> = vec![];
        loop {
            if ordering.len() == n {
                break;
            }
            if vs.len() == 0 {
                ordering = vec![];
                break;
            }

            let v = vs.pop().unwrap();
            ordering.push(v);

            for j in 0..n {
                if ma[j][v] {
                    ma[j][v] = false;
                    incomings[j] -= 1;
                    if incomings[j] == 0 {
                        vs.push(j);
                    }
                }
            }
        }

        ordering.iter().map(|&x| x as i32).collect()
    }
}

struct Solution {}