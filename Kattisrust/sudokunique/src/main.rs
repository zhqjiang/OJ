use std::io::{self, BufRead};
struct Board {
    data: [[u8; 9]; 9],
}

fn check(a: [u8; 9]) {}

impl Board {
    fn check_error(&self) {
        for i in 0..9 {
            check(self.data[i]);
        }
        for i in 0..9 {
            check([
                self.data[0][i],
                self.data[1][i],
                self.data[2][i],
                self.data[3][i],
                self.data[4][i],
                self.data[5][i],
                self.data[6][i],
                self.data[7][i],
                self.data[8][i],
            ]);
        }

        for i in 0..3 {
            for j in 0..3 {
                check([
                    self.data[0 + i * 3][0 + j + 3],
                    self.data[0 + i * 3][1 + j + 3],
                    self.data[0 + i * 3][2 + j + 3],
                    self.data[1 + i * 3][0 + j + 3],
                    self.data[1 + i * 3][1 + j + 3],
                    self.data[1 + i * 3][2 + j + 3],
                    self.data[2 + i * 3][0 + j + 3],
                    self.data[2 + i * 3][1 + j + 3],
                    self.data[2 + i * 3][2 + j + 3],
                ])
            }
        }
    }
}

fn solve(board: Board) {}

fn main() {
    let mut board = Board { data: [[0; 9]; 9] };
    let mut counts: usize = 0;

    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let s = line.trim();
        if s == "" {
            counts = 0;
        } else {
            let nums: Vec<u8> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            for i in 0..9 {
                board.data[counts][i] = nums[i];
            }
            counts += 1;
            if counts == 9 {
                solve(Board { data: board.data });
            }
        }
    }
}
