impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k_usize = k as usize;
        if s.len() < k_usize {
            return false;
        }

        let mut rec = vec![false; 1 << k_usize];
        for i in 0..s.len() {
            if i + k_usize > s.len() {
                continue;
            }
            let window_str = &s[i..i + k_usize];
            let window_integer = usize::from_str_radix(window_str, 2).unwrap();
            rec[window_integer] = true;
        }

        rec.into_iter().all(|x| x)
    }
}
struct Solution;
fn main() {
    assert_eq!(Solution::has_all_codes(String::from("00110110"), 2), true);
    assert_eq!(Solution::has_all_codes(String::from("0110"), 1), true);
    assert_eq!(Solution::has_all_codes(String::from("0110"), 2), false);
    assert_eq!(Solution::has_all_codes(String::from("00110"), 2), true);
}
