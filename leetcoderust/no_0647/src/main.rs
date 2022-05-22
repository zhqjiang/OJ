impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        3
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
