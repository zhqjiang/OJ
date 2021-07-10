#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(iq_test("2 4 7 8 10"), 3);
        assert_eq!(iq_test("1 2 2"), 1);
    }
}

fn iq_test(numbers: &str) -> u64 {
    let mut odd: u64 = 0;
    let mut even: u64 = 0;
    let mut result = 0;

    for (idx, number) in numbers
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .enumerate()
    {
        if (number % 2 == 1) && even >= 2 {
            result = idx as u64;
            break;
        }
        if (number % 2 == 0) && odd >= 2 {
            result = idx as u64;
            break;
        }
        odd += number % 2;
        even += (number + 1) % 2;
    }

    result + 1
}
