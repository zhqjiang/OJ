use std::io::{self, BufRead};

fn sort(v: &mut Vec<(u8, usize)>) {
    for i in 0..v.len() {
        for j in (0..i).rev() {
            if v[j].1 > v[j + 1].1 {
                v.swap(j, j + 1);
            } else if v[j].1 == v[j + 1].1 {
                if v[j].0 > v[j + 1].0 {
                    v.swap(j, j + 1);
                }
            }
        }
    }
}

fn find_suitable_value_two(record: &mut Vec<(u8, usize)>, v1: u8, v2: u8) -> u8 {
    let mut res: u8 = 0;
    for item in record {
        if (item.0 != v1) && (item.0 != v2) {
            res = item.0;
            item.1 += 1;
            break;
        }
    }
    res
}

fn find_suitable_value_three(record: &mut Vec<(u8, usize)>, v1: u8, v2: u8, v3: u8) -> u8 {
    let mut res: u8 = 0;
    for item in record {
        if ((item.0 != v1) && (item.0 != v2)) && (item.0 != v3) {
            res = item.0;
            item.1 += 1;
            break;
        }
    }
    res
}

fn gen(number: i16) -> u8 {
    let number = number as usize;

    let mut head: usize = 0;
    let mut occur_recording: Vec<(u8, usize)> = vec![(0, 1), (1, 1), (2, 0), (3, 0), (4, 0)]; // (value, occur_times)

    let mut tiles: Vec<(u8, u8)> = vec![(5, 0), (5, 1)]; // (left_edges, value)

    let mut i: usize = 2;

    while i < number {
        match tiles[head] {
            (1, _) => {
                let new_value = find_suitable_value_three(
                    &mut occur_recording,
                    tiles[i - 1].1,
                    tiles[head].1,
                    tiles[head + 1].1,
                );

                sort(&mut occur_recording);

                tiles[head].0 = 0;
                tiles[head + 1].0 -= 1;
                if i >= 2 {
                    let k = tiles.len() - 1;
                    tiles[k].0 = tiles[k].0 - 1;
                };
                tiles.push((3, new_value));

                i += 1;
            }
            (0, _) => {
                head += 1;
            }
            (left_edges, _) => {
                let new_value =
                    find_suitable_value_two(&mut occur_recording, tiles[i - 1].1, tiles[head].1);
                sort(&mut occur_recording);

                tiles[head].0 = left_edges - 1;
                let k = tiles.len() - 1;
                tiles[k].0 = tiles[k].0 - 1;
                tiles.push((4, new_value));

                i += 1;
            }
        }
        println!("{:?}", tiles);
    }

    tiles[tiles.len() - 1].1
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");

    let mut numbers: Vec<i16> = Vec::new();
    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        match line.trim().parse::<i16>() {
            Ok(x) => numbers.push(x),
            Err(_) => {}
        };
    }

    for item in numbers {
        if item == 1 {
            println!("1");
        } else {
            println!("{}", gen(item) + 1);
        }
    }
}
