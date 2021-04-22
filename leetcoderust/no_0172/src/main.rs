impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let five_one = n / 5;
        let five_two = n / 25;
        let five_three = n / 125;
        let five_four = n / 625;
        let five_five = n / 3125;

        let only_one_five = five_one - five_two;
        let only_two_five = five_two - five_three;
        let only_three_five = five_three - five_four;
        let only_four_five = five_four - five_five;

        only_one_five + only_two_five * 2 + only_three_five * 3 + only_four_five * 4 + five_five * 5
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::trailing_zeroes(3), 0);
    assert_eq!(Solution::trailing_zeroes(5), 1);
    assert_eq!(Solution::trailing_zeroes(0), 0);
    assert_eq!(Solution::trailing_zeroes(25), 6);
    assert_eq!(Solution::trailing_zeroes(133), 32);
}
