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

struct Tiles {
    data: [u8; 10000],
    head: usize,
    end: usize,
}

impl Tiles {
    fn new() -> Tiles {
        let mut v = [0; 10000];
        v[0] = 5 * 8 + 0; // left_edges * 8 + value
        v[1] = 5 * 8 + 1;
        Tiles {
            data: v,
            head: 0,
            end: 1,
        }
    }

    fn head_value(&self) -> u8 {
        self.data[self.head] % 8
    }
    fn second_value(&self) -> u8 {
        self.data[self.head + 1] % 8
    }
    fn tail_value(&self) -> u8 {
        let x = self.data[self.end] % 8;
        x
    }
    fn head_left_edges(&self) -> u8 {
        self.data[self.head] / 8
    }

    fn move_head_forward(&mut self) {
        self.head += 1;
    }
    fn move_end_forward(&mut self) {
        self.end += 1;
    }
    fn push(&mut self, value: u8, left_edges: u8) {
        self.move_end_forward();
        self.data[self.end] = value + left_edges * 8;
    }
    fn decrease(&mut self, index: usize) {
        self.data[index] -= 8;
    }
}

fn gen(number: i16) -> u8 {
    let mut occur_recording: Vec<(u8, usize)> = vec![(0, 1), (1, 1), (2, 0), (3, 0), (4, 0)]; // (value, occur_times)

    let mut tiles = Tiles::new();

    let mut i: usize = 2;
    while i < number as usize {
        match tiles.head_left_edges() {
            1 => {
                let new_value = find_suitable_value_three(
                    &mut occur_recording,
                    tiles.tail_value(),
                    tiles.head_value(),
                    tiles.second_value(),
                );

                sort(&mut occur_recording);

                tiles.decrease(tiles.head);
                tiles.decrease(tiles.head + 1);
                tiles.decrease(tiles.end);
                tiles.push(new_value, 3);

                i += 1;
            }
            0 => tiles.move_head_forward(),
            _ => {
                let new_value = find_suitable_value_two(
                    &mut occur_recording,
                    tiles.tail_value(),
                    tiles.head_value(),
                );
                sort(&mut occur_recording);
                tiles.decrease(tiles.head);
                tiles.decrease(tiles.end);
                tiles.push(new_value, 4);

                i += 1;
            }
        }
    }
    tiles.tail_value()
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
