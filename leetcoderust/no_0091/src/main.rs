impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut counts_two = 0;
        let mut counts_one = if s_chars[0] != '0' { 1 } else { 0 };

        for i in 1..s_chars.len() {
            let tmp = counts_one;
            counts_one = if s_chars[i] > '0' {
                counts_two + counts_one
            } else {
                0
            };

            counts_two = if (s_chars[i - 1] == '1') || (s_chars[i - 1] == '2' && s_chars[i] <= '6')
            {
                tmp
            } else {
                0
            };
        }

        counts_two + counts_one
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::num_decodings(String::from("12")), 2);
    assert_eq!(Solution::num_decodings(String::from("226")), 3);
    assert_eq!(Solution::num_decodings(String::from("27")), 1);
}
