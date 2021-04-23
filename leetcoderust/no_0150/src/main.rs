impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];

        for item in tokens {
            match item.as_str() {
                "+" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    let value = v2 + v1;
                    stack.push(value);
                }
                "-" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    let value = v2 - v1;
                    stack.push(value);
                }
                "*" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    let value = v2 * v1;
                    stack.push(value);
                }
                "/" => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    let value = v2 / v1;
                    stack.push(value);
                }
                _ => {
                    let value = item.parse::<i32>().unwrap();
                    stack.push(value);
                }
            }
        }

        stack[0]
    }
}
struct Solution {}
fn main() {
    let v = vec![
        String::from("10"),
        String::from("6"),
        String::from("9"),
        String::from("3"),
        String::from("+"),
        String::from("-11"),
        String::from("*"),
        String::from("/"),
        String::from("*"),
        String::from("17"),
        String::from("+"),
        String::from("5"),
        String::from("+"),
    ];
    let v2 = vec![
        String::from("4"),
        String::from("13"),
        String::from("5"),
        String::from("/"),
        String::from("+"),
    ];

    assert_eq!(Solution::eval_rpn(v), 22);
    assert_eq!(Solution::eval_rpn(v2), 6);
}
