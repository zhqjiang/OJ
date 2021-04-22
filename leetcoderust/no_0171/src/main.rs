impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let a_u8 = 'A' as u8;
        let mut base: i64 = 1;
        let mut res = 0;

        let mut cs: Vec<u8> = column_title.as_bytes().iter().map(|&x| x).collect();

        loop {
            if cs.len() == 0 {
                break;
            }
            let c_u8 = cs.pop().unwrap();
            res += (c_u8 - a_u8 + 1) as i64 * base;
            base *= 26;
        }

        res as i32
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::title_to_number(String::from("A")), 1);
    assert_eq!(Solution::title_to_number(String::from("AB")), 28);
    assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
    assert_eq!(
        Solution::title_to_number(String::from("FXSHRXW")),
        2147483647
    );
}
