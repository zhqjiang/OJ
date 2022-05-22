// This solution is too slow

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let length = s.len();
        let mut records: Vec<Vec<bool>> = vec![vec![false; length]; length];

        let bytes = s.as_bytes();

        for i in 0..length {
            records[i][i] = true;
        }

        let mut count = 0;

        for len in 2..=length {
            for start in 0..length {
                let end = start + len - 1;
                if end >= length {
                    break;
                }
                if bytes[start] == bytes[end] && (start == end - 1 || records[start + 1][end - 1]) {
                    records[start][end] = true;
                    // records[end][start] = true;
                    count += 1;
                }
            }
        }

        count + (length as i32)
    }
}

struct Solution {}
fn main() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    assert_eq!(Solution::count_substrings("aba".to_string()), 4);
    assert_eq!(Solution::count_substrings("a".to_string()), 1);
    assert_eq!(Solution::count_substrings("b".to_string()), 1);
}
