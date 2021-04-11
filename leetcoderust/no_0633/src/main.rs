fn is_square_num(num: i32) -> bool {
    let root_guess = get_floor_root(num);
    let root_guess_2 = root_guess - 1;
    (root_guess * root_guess == num) || (root_guess_2 * root_guess_2 == num)
}

fn get_floor_root(num: i32) -> i32 {
    let num_float = num as f64;
    num_float.sqrt().floor() as i32
}

pub fn judge_square_sum(c: i32) -> bool {
    let max_root = get_floor_root(c);

    let mut is_square_sum: bool = false;

    for a in 0..max_root + 1 {
        if is_square_num(c - a * a) {
            is_square_sum = true;
            break;
        }
    }

    is_square_sum
}

fn main() {
    println!("{}", judge_square_sum(5));
    println!("{}", judge_square_sum(3));
    println!("{}", judge_square_sum(4));
    println!("{}", judge_square_sum(1));
    println!("{}", judge_square_sum(1));
}
