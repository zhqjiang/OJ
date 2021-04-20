/*
use another implementation(HashMap rather than Vec) of trie will save memory usage.
*/
#[derive(Clone)]
struct TrieNode {
    next: Vec<Option<TrieNode>>,
    word: Option<String>,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            next: vec![None; 26],
            word: None,
        }
    }
    fn add(&mut self, word: &String) {
        self.add_char(0, word);
    }
    fn add_char(&mut self, idx: usize, word: &String) {
        if idx == word.as_bytes().len() {
            self.word = Some(word.clone());
            return;
        }

        let c: char = word.chars().clone().nth(idx).unwrap();
        let c_idx: usize = (|x: char| (x as u8 - 'a' as u8) as usize)(c);

        match self.next[c_idx].as_mut() {
            Some(x) => {
                x.add_char(idx + 1, word);
            }
            None => {
                let mut new_node = TrieNode::new();
                new_node.add_char(idx + 1, word);
                self.next[c_idx] = Some(new_node);
            }
        }
    }
}

fn dfs(i: usize, j: usize, root: &mut TrieNode, res: &mut Vec<String>, board: &mut Vec<Vec<char>>) {
    let c = board[i][j];
    if c == '#' {
        return;
    }

    let c_idx = (|x: char| (x as u8 - 'a' as u8) as usize)(c);
    if let Some(child) = root.next[c_idx].as_mut() {
        if let Some(w) = child.word.as_mut() {
            res.push(w.clone());
            child.word = None;
        }

        board[i][j] = '#';
        if i > 0 {
            dfs(i - 1, j, child, res, board);
        }
        if (i + 1) < board.len() {
            dfs(i + 1, j, child, res, board);
        }
        if j > 0 {
            dfs(i, j - 1, child, res, board);
        }
        if (j + 1) < board[0].len() {
            dfs(i, j + 1, child, res, board);
        }
        board[i][j] = c;
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut b = board.clone();
        let mut res = vec![];
        let mut root = TrieNode::new();
        for word in &words {
            root.add(word);
        }

        let m = board.len();
        let n = board[0].len();

        for i in 0..m {
            for j in 0..n {
                dfs(i, j, &mut root, &mut res, &mut b);
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
                String::from("rain"),
                String::from("oathfi")
            ]
        ),
        vec![String::from("oath"), String::from("oathfi")]
    );
}
