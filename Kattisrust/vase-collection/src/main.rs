use std::io;

struct M {
    matrices: [[u8; 36]; 36],
    columns_sums: [u8; 36],
    rows_sums: [u8; 36],
    size: u8,
    k: u8,
    k_confirmed: bool,
}

fn n_choose_t_util(res: &mut Vec<Vec<u8>>, tmp: &mut Vec<u8>, n: usize, left: usize, target: u8) {
    if target == 0 {
        res.push(tmp.clone());
    }
    if target > 0 {
        for i in left..n {
            tmp.push(i as u8);
            n_choose_t_util(res, tmp, n, i + 1, target - 1);

            tmp.pop();
        }
    }
}

fn n_choose_t(n: usize, target: u8) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = Vec::new();
    let mut tmp: Vec<u8> = Vec::new();
    n_choose_t_util(&mut res, &mut tmp, n, 0, target);
    res
}

impl M {
    fn clean_row(&mut self, row_idx: usize) {
        let this_row_sum = self.rows_sums[row_idx];
        if (this_row_sum <= self.size) && (this_row_sum > 0) {
            for column_idx in 0..36 {
                self.erase(row_idx, column_idx)
            }
        }
    }

    fn clean_column(&mut self, column_idx: usize) {
        let this_column_sum = self.columns_sums[column_idx];
        if (this_column_sum <= self.size) && (this_column_sum > 0) {
            for row_idx in 0..36 {
                self.erase(row_idx, column_idx)
            }
        }
    }

    fn erase(&mut self, row_idx: usize, column_idx: usize) {
        if self.matrices[row_idx][column_idx] == 1 {
            self.matrices[row_idx][column_idx] = 0;
            self.rows_sums[row_idx] -= 1;
            self.columns_sums[column_idx] -= 1;
            self.clean_row(row_idx);
            self.clean_column(column_idx);
        }
    }

    fn clean(&mut self) {
        loop {
            for i in 0..36 {
                self.clean_row(i);
            }
            for j in 0..36 {
                self.clean_column(j);
            }

            if self.is_all_empty() || self.k_confirmed {
                break;
            } else {
                self.update_k();
                self.size = self.size + 1;
            }
        }
    }

    fn update_k(&mut self) {
        let target = self.k + 1;

        self.k_confirmed = true;
        let mut r = Vec::new();
        let mut c = Vec::new();
        for (i, &v) in self.rows_sums.iter().enumerate() {
            if v > 0 {
                r.push(i as u8);
            }
        }
        for (i, &v) in self.columns_sums.iter().enumerate() {
            if v > 0 {
                c.push(i as u8);
            }
        }
        let row_cs: Vec<Vec<u8>> = n_choose_t(r.len(), target);
        let column_cs: Vec<Vec<u8>> = n_choose_t(c.len(), target);

        let mut find_one = false;
        for i in &row_cs {
            for j in &column_cs {

                let mut this_one_works = true;
                for &row_idx in i {
                    for &col_idx in j {
                        let real_row_idx = r[row_idx as usize] as usize;
                        let real_col_idx = c[col_idx as usize] as usize;

                        if self.matrices[real_row_idx][real_col_idx] == 0 {
                            this_one_works = false;
                        }
                        if !this_one_works {
                            break;
                        }
                    }
                    if !this_one_works {
                        break;
                    }
                }
                if this_one_works {
                    find_one = true;
                }
                if find_one {
                    break;
                }
            }
            if find_one {
                break;
            }
        }

        if !find_one {
            self.k_confirmed = true;
        } else {
            self.k = self.k + 1;
            self.k_confirmed = false;
        }
    }

    fn is_all_empty(&self) -> bool {
        let mut res = true;

        for &item in self.rows_sums.iter() {
            if item > 0 {
                res = false
            }
        }

        res
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let counts = buffer.trim().parse::<i16>().unwrap();

    for _ in 0..counts {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let m = buffer.trim().parse::<usize>().unwrap();

        let mut ma = M {
            matrices: [[0; 36]; 36],
            rows_sums: [0; 36],
            columns_sums: [0; 36],
            size: 1,
            k: 1,
            k_confirmed: false,
        };

        for _ in 0..m {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            let nums: Vec<usize> = buffer
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let row_idx = nums[0] - 1;
            let column_idx = nums[1] - 1;
            if ma.matrices[row_idx][column_idx] == 0 {
                ma.matrices[row_idx][column_idx] = 1;
                ma.rows_sums[row_idx] += 1;
                ma.columns_sums[column_idx] += 1;
            }
        }
        if m <= 3 {
            println!("1");
        } else {
            ma.clean();
            println!("{}", ma.k);
        }
    }
}
