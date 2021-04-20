#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec![
                    String::from("oath"),
                    String::from("pea"),
                    String::from("eat"),
                    String::from("rain")
                ]
            ),
            vec![String::from("oath"), String::from("eat")]
        );
    }
}

use std::str::Chars;

fn find(
    i: usize,
    j: usize,
    used: &mut Vec<Vec<bool>>,
    idx: usize,
    word: &Chars,
    board: &Vec<Vec<char>>,
) -> bool {
    if idx == word.as_str().len() {
        return true;
    }

    let m = board.len();
    let n = board[0].len();
    let mut res = false;
    let c = word.clone().nth(idx).unwrap();

    let mut neighbors = vec![];
    if j > 0 {
        let neighbor = board[i][j - 1];
        if neighbor == c && !used[i][j - 1] {
            neighbors.push((i, j - 1));
        }
    }
    if j < n - 1 {
        let neighbor = board[i][j + 1];
        if neighbor == c && !used[i][j + 1] {
            neighbors.push((i, j + 1));
        }
    }
    if i > 0 {
        let neighbor = board[i - 1][j];
        if neighbor == c && !used[i - 1][j] {
            neighbors.push((i - 1, j));
        }
    }
    if i < m - 1 {
        let neighbor = board[i + 1][j];
        if neighbor == c && !used[i + 1][j] {
            neighbors.push((i + 1, j));
        }
    }

    for loc in neighbors {
        used[loc.0][loc.1] = true;
        res = res || find(loc.0, loc.1, used, idx + 1, word, board);
        used[loc.0][loc.1] = false;
    }

    res
}

fn find_word(word: &String, board: &Vec<Vec<char>>) -> bool {
    let m = board.len();
    let n = board[0].len();

    let mut used: Vec<Vec<bool>> = vec![vec![false; n]; m];

    let mut res = false;
    for i in 0..m {
        for j in 0..n {
            let c = word.chars().clone().nth(0).unwrap();
            if board[i][j] == c {
                used[i][j] = true;
                res = res || find(i, j, &mut used, 1, &word.chars(), board);
                used[i][j] = false;
            }
        }
    }

    res
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for word in &words {
            if find_word(&word, &board) {
                res.push(word.clone());
            }
        }
        res
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        Solution::find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v']
            ],
            vec![
                String::from("oath"),
                String::from("pea"),
                String::from("eat"),
                String::from("rain")
            ]
        ),
        vec![String::from("oath"), String::from("eat")]
    );
}
