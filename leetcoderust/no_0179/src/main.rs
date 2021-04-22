use std::cmp::Ordering;

fn compare(a: u8, b: u8, nums: &Vec<i32>) -> Ordering {
    let a = nums[a as usize].to_string();
    let a: Vec<char> = a.chars().collect();
    let b = nums[b as usize].to_string();
    let b: Vec<char> = b.chars().collect();

    let len_a = a.len();
    let len_b = b.len();

    let mut order = Ordering::Equal;

    for i in 0..(len_a + len_b) {
        let char_a = if i < len_a { a[i] } else { b[i - len_a] };
        let char_b = if i < len_b { b[i] } else { a[i - len_b] };
        if char_a > char_b {
            order = Ordering::Less;
            break;
        } else if char_a == char_b {
            continue;
        } else {
            order = Ordering::Greater;
            break;
        }
    }
    order
}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut idxs = nums
            .iter()
            .enumerate()
            .map(|(x, _)| x as u8)
            .collect::<Vec<u8>>();
        idxs.sort_by(|&a, &b| compare(a, b, &nums));
        let first = nums[idxs[0] as usize];
        if first == 0 {
            return String::from("0");
        }

        idxs.iter()
            .map(|&x| nums[x as usize].to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::largest_number(vec![0, 9]), String::from("90"));
    assert_eq!(
        Solution::largest_number(vec![3, 30, 34, 5, 9]),
        String::from("9534330")
    );
    assert_eq!(Solution::largest_number(vec![1]), String::from("1"));
    assert_eq!(Solution::largest_number(vec![10]), String::from("10"));
    assert_eq!(
        Solution::largest_number(vec![34323, 3432]),
        String::from("343234323")
    );
}
