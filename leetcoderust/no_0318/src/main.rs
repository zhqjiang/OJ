impl Solution {
    pub fn calc_chars_freq(word: &String) -> [bool; 26] {
        let mut res = [false; 26];
        for c in word.chars() {
            let idx = c as u8 - 'a' as u8;
            res[idx as usize] = true;
        }
        res
    }

    pub fn is_overlap(word_1: [bool; 26], word_2: [bool; 26]) -> bool {
        for i in 0..26 {
            if word_1[i] && word_2[i] {
                return true;
            }
        }
        false
    }

    pub fn max_product(words: Vec<String>) -> i32 {
        let length = words.len();
        let records: Vec<([bool; 26], i32)> = words
            .iter()
            .map(|word| return (Solution::calc_chars_freq(word), word.len() as i32))
            .collect();

        let mut res = 0;
        for i in 0..length {
            for j in 0..length {
                if i == j {
                    continue;
                } else {
                    let product = records[i].1 * records[j].1;
                    if product <= res {
                        continue;
                    }
                    if !Solution::is_overlap(records[i].0, records[j].0) {
                        res = product;
                    }
                }
            }
        }

        res
    }
}

struct Solution {}
fn main() {
    assert_eq!(
        Solution::max_product(
            (vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"])
                .iter()
                .map(|x| x.to_string())
                .collect()
        ),
        16
    );
    assert_eq!(
        Solution::max_product(
            (vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"])
                .iter()
                .map(|x| x.to_string())
                .collect()
        ),
        4
    );
    assert_eq!(
        Solution::max_product(
            (vec!["a", "aa", "aaa", "aaaa"])
                .iter()
                .map(|x| x.to_string())
                .collect()
        ),
        0
    );
}
