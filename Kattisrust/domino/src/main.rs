/*
 *[Domino](https://open.kattis.com/problems/domino)
* meet in the middle
* max clique
* use i64 instead of i32, which will results in overflow
*/
use std::collections::HashMap;
use std::io;
type Domino = (usize, usize, u8, i64);
fn max(n1: i64, n2: i64) -> i64 {
    let sum = if n1 > n2 { n1 } else { n2 };
    sum
}
fn min(n1: usize, n2: usize) -> usize {
    let sum = if n1 < n2 { n1 } else { n2 };
    sum
}
struct MaskIter(u64, usize);
impl Iterator for MaskIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < 64 {
            if ((1 << self.1) & self.0) > 0 {
                self.1 += 1;
                return Some(self.1 - 1);
            } else {
                self.1 += 1;
                return self.next();
            }
        } else {
            None
        }
    }
}

// only top 7 * (k - 1) + 1 pairs among all pairs are allowed to be placed dominos
fn gen_allowed_pairs(matrix: &Vec<Vec<i64>>, k: usize) -> Vec<Domino> {
    let mut res: Vec<Domino> = vec![];
    let n = matrix.len();
    let allowed_size = min(k * 7 - 6, n * n * 2 - n * 2);

    for i in 0..n {
        for j in 0..n {
            // horizontal, 0 of (i, j, 0, value) means horizontal
            if i < n - 1 {
                let value = matrix[i][j] + matrix[i + 1][j];
                if res.len() > allowed_size {
                    if value > res[res.len() - 1].3 {
                        match res.binary_search_by(|(.., v)| value.cmp(&v)) {
                            Ok(pos) => res.insert(pos, (i, j, 0, value)),
                            Err(pos) => res.insert(pos, (i, j, 0, value)),
                        }
                    }
                } else {
                    match res.binary_search_by(|(.., v)| value.cmp(&v)) {
                        Ok(pos) => res.insert(pos, (i, j, 0, value)),
                        Err(pos) => res.insert(pos, (i, j, 0, value)),
                    }
                }
            }

            // vertical, 1 of (i, j, 1, value) means vertical
            if j < n - 1 {
                let value = matrix[i][j] + matrix[i][j + 1];
                if res.len() > allowed_size {
                    if value > res[res.len() - 1].3 {
                        match res.binary_search_by(|(.., v)| value.cmp(&v)) {
                            Ok(pos) => res.insert(pos, (i, j, 1, value)),
                            Err(pos) => res.insert(pos, (i, j, 1, value)),
                        }
                    }
                } else {
                    match res.binary_search_by(|(.., v)| value.cmp(&v)) {
                        Ok(pos) => res.insert(pos, (i, j, 1, value)),
                        Err(pos) => res.insert(pos, (i, j, 1, value)),
                    }
                }
            }

            while res.len() > allowed_size {
                res.pop();
            }
        }
    }

    res
}

fn overlap(domi_a: &Domino, domi_b: &Domino) -> bool {
    let a_1 = (domi_a.0, domi_a.1);
    let a_2 = (
        domi_a.0 + 1 - domi_a.2 as usize,
        domi_a.1 + domi_a.2 as usize,
    );
    let b_1 = (domi_b.0, domi_b.1);
    let b_2 = (
        domi_b.0 + 1 - domi_b.2 as usize,
        domi_b.1 + domi_b.2 as usize,
    );

    if a_1 == b_1 || a_1 == b_2 || a_2 == b_1 || a_2 == b_2 {
        return true;
    } else {
        return false;
    }
}

fn calculate(number: u64, pairs: &Vec<Domino>) -> i64 {
    let mut res = 0;
    for i in MaskIter(number, 0) {
        res += pairs[i].3;
    }
    res
}

fn s_cliques(at: usize, t: usize, cli: u64, right: usize, res: &mut Vec<u64>, matrix: &Vec<u64>) {
    if t == 0 {
        res.push(cli);
        return;
    } else {
        if at == right {
            return;
        }
    }

    if (cli & matrix[at]) == 0 {
        s_cliques(at + 1, t - 1, cli | (1 << at), right, res, matrix);
    }
    s_cliques(at + 1, t, cli, right, res, matrix);
}

// search all cliques in this part while its size is exactly t
// use bitmask to represent a clique
fn get_cliques(left: usize, right: usize, t: usize, matrix: &Vec<u64>) -> Vec<u64> {
    let mut res = vec![];
    if (right - left) < t {
        return res;
    }
    s_cliques(left, t, 0, right, &mut res, matrix);
    res
}

fn cal_sum(
    at: usize,
    right: usize,
    t: usize,
    cliq: u64,
    matrix: &Vec<u64>,
    p: &Vec<Domino>,
) -> i64 {
    if t == 0 {
        return 0;
    } else {
        if at == right {
            return 0;
        }
    }

    let mut res = 0;

    if (cliq & matrix[at]) == 0 {
        res = max(
            res,
            p[at].3 + cal_sum(at + 1, right, t - 1, cliq | 1 << at, matrix, p),
        );
    }
    res = max(res, cal_sum(at + 1, right, t, cliq, matrix, p));

    res
}

fn cal_sum_special(
    at: usize,
    right: usize,
    t: usize,
    cliq: u64,
    matrix: &Vec<u64>,
    p: &Vec<Domino>,
) -> i64 {
    if t > right - at {
        return 0;
    }
    if t == 0 {
        return 0;
    } else {
        if at == right {
            return 0;
        }
    }

    let mut res = 0;

    if (cliq & matrix[at]) == 0 {
        res = max(
            res,
            p[at].3 + cal_sum_special(at + 1, right, t - 1, cliq | 1 << at, matrix, p),
        );
    }

    res = max(res, cal_sum_special(at + 1, right, t, cliq, matrix, p));

    res
}

fn split_solve(pairs: &Vec<Domino>, matrix: &Vec<u64>, k: usize) -> i64 {
    let middle = pairs.len() * 2 / 5;
    let mut sum = 0;
    for t in 0..k + 1 {
        // check whether the clique of first part with the clique of second part
        // is able to generate a new clique
        if t == 0 {
            sum = max(sum, cal_sum(middle, pairs.len(), k, 0, matrix, pairs));
        } else if t == k {
            sum = max(sum, cal_sum(0, middle, k, 0, matrix, pairs));
        } else {
            let first_cliques = get_cliques(0, middle, t, matrix);
            let mut m: HashMap<u64, i64> = HashMap::new();

            for &i in &first_cliques {
                let sum_first = calculate(i, pairs);
                let mut optional: u64 = 0;
                for j in middle..pairs.len() {
                    if i & matrix[j] == 0 {
                        optional = optional | 1 << j;
                    }
                }

                match m.get(&optional) {
                    Some(x) => {
                        if sum_first > *x {
                            m.insert(optional, sum_first);
                            sum = max(
                                sum,
                                sum_first
                                    + cal_sum_special(middle, pairs.len(), k - t, i, matrix, pairs),
                            );
                        }
                    }
                    None => {
                        m.insert(optional, sum_first);
                        sum = max(
                            sum,
                            sum_first
                                + cal_sum_special(middle, pairs.len(), k - t, i, matrix, pairs),
                        );
                    }
                }
            }
        }
    }

    sum
}

fn brute(pairs: &Vec<Domino>, matrix: &Vec<u64>, k: usize) -> i64 {
    cal_sum(0, pairs.len(), k, 0, matrix, pairs)
}

fn run(matrix: &Vec<Vec<i64>>, k: usize, counts: i64) {
    let n = matrix.len();
    let pairs = gen_allowed_pairs(matrix, k);

    // map all domino possible locations to a new graph
    // if two locations overlap, they are not connected
    let mut n_matrix: Vec<u64> = vec![0; pairs.len()];
    for i in 0..pairs.len() {
        for j in 0..i {
            if overlap(&pairs[i], &pairs[j]) {
                n_matrix[i] = n_matrix[i] | (1 << j);
                n_matrix[j] = n_matrix[j] | (1 << i);
            }
        }
    }

    if k <= 4 || ((k * 7 - 6) >= (n * n * 2 - n * 2)) {
        println!("{}", counts - brute(&pairs, &n_matrix, k));
    } else {
        println!("{}", counts - split_solve(&pairs, &n_matrix, k));
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let nums: Vec<usize> = buffer
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let n: usize = nums[0];
    let k: usize = nums[1];
    let mut counts = 0;

    let mut matrix: Vec<Vec<i64>> = vec![Vec::new(); n];
    for i in 0..n {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let nums: Vec<i64> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        counts += nums.iter().fold(0, |acc, &x| acc + x);
        matrix[i] = nums;
    }
    run(&matrix, k, counts);
}
