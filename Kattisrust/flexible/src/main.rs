#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            generate(10, vec![1, 4, 8]),
            vec![1, 2, 3, 4, 6, 7, 8, 9, 10]
        );
        assert_eq!(generate(6, vec![2, 5]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(generate(6, vec![3]), vec![3, 6]);
        assert_eq!(generate(5, vec![1, 2]), vec![1, 2, 3, 4, 5]);
    }
}

use std::io;

fn generate(width: i16, vec: Vec<i16>) -> Vec<i16> {
    let mut rec = [false; 101];
    rec[width as usize] = true;

    for i in 0..vec.len() {
        rec[vec[i] as usize] = true;
        rec[(width - vec[i]) as usize] = true;
        for j in 0..i {
            rec[(vec[i] - vec[j]) as usize] = true;
        }
    }

    rec.iter()
        .enumerate()
        .filter_map(|(index, &is_ok)| if is_ok { Some(index as i16) } else { None }) // is_ok.then(|| index as i16)
        .collect()
}

fn main() {
    let mut line1 = String::new();
    match io::stdin().read_line(&mut line1) {
        Ok(_) => {}
        Err(_) => {}
    }
    let nums: Vec<i16> = line1
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let width = nums[0];
    let _partition_number = nums[1];

    let mut line2 = String::new();
    match io::stdin().read_line(&mut line2) {
        Ok(_) => {}
        Err(_) => {}
    }
    let nums: Vec<i16> = line2
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let result = generate(width, nums);
    let k: Vec<String> = result.iter().map(|&x| x.to_string()).collect();

    println!("{}", k.join(" "));
}
