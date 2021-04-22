impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut primes = vec![true; n];
        match n {
            0 | 1 | 2 => {
                return 0;
            }
            3 => {
                return 1;
            }
            4 | 5 => {
                return 2;
            }
            6 | 7 => {
                return 3;
            }
            8 | 9 => {
                return 4;
            }
            _ => {
                primes[0] = false;
                primes[1] = false;
                primes[2] = true;
                primes[3] = true;
                primes[4] = false;
                primes[5] = true;
                primes[6] = false;
                primes[7] = true;
                primes[8] = false;

                for i in 2..(n + 1) / 2 {
                    let k = i * 2;
                    if k < n {
                        primes[k] = false;
                    }
                }

                let root = (n as f32).sqrt() as usize;

                for i in 3..=root {
                    if primes[i] {
                        let mut j = i * 3;

                        while j < n {
                            primes[j] = false;
                            j += i << 1;
                        }
                    }
                }
            }
        }

        primes.iter().fold(0, |acc, &x| acc + if x { 1 } else { 0 })
    }
}
struct Solution {}
fn main() {
    assert_eq!(Solution::count_primes(100), 25);
}
