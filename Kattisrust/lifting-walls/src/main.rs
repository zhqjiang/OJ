use std::{io, usize};

struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn included(&self, another: &Point, distance: i16) -> bool {
        (self.x - another.x).pow(2) + (self.y - another.y).pow(2) <= distance.pow(2)
    }
}
#[derive(Debug)]
struct Relations {
    data: Vec<(u8, u8, u8, u8)>,
}

impl Relations {
    fn generate(&self, relation: (u8, u8, u8, u8)) -> Relations {
        for item in &self.data {
            if item.0 >= relation.0
                && item.1 >= relation.1
                && item.2 >= relation.2
                && item.3 >= relation.3
            {
                return Relations {
                    data: self.data.clone(),
                };
            }
        }

        let mut new_data = Vec::new();
        for item in &self.data {
            if !(item.0 <= relation.0
                && item.1 <= relation.1
                && item.2 <= relation.2
                && item.3 <= relation.3)
            {
                new_data.push(*item);
            }
        }
        new_data.push(relation);

        Relations { data: new_data }
    }

    fn result(&self) -> String {
        let mut min = 4;
        let mut is_possible = false;
        let mut combination: Vec<Vec<usize>> = Vec::new();

        for i in 0..self.data.len() {
            let mut to_be_pushed = Vec::new();
            for old in &combination {
                let mut new = old.clone();
                new.push(i);
                to_be_pushed.push(new);
            }
            for i in to_be_pushed {
                combination.push(i);
            }
            combination.push(vec![i]);
        }

        for m in &combination {
            let mut sum0: u8 = 0;
            let mut sum1: u8 = 0;
            let mut sum2: u8 = 0;
            let mut sum3: u8 = 0;
            for &n in m {
                sum0 += self.data[n].0;
                sum1 += self.data[n].1;
                sum2 += self.data[n].2;
                sum3 += self.data[n].3;
            }
            if sum0 > 0 && sum1 > 0 && sum2 > 0 && sum3 > 0 {
                is_possible = true;
                if m.len() < min {
                    min = m.len()
                }
            }
        }

        if is_possible {
            format!("{}", min)
        } else {
            String::from("Impossible")
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let nums: Vec<i16> = buffer
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let l = nums[0];
    let w = nums[1];
    let distance = nums[3] * 2;
    let centers = vec![
        Point { x: l, y: 0 },
        Point { x: -l, y: 0 },
        Point { x: 0, y: w },
        Point { x: 0, y: -w },
    ];

    let mut relations = Relations {
        data: Vec::with_capacity(4),
    };
    let possible_loc_count = nums[2];
    for _ in 0..possible_loc_count {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let nums: Vec<i16> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let p = Point {
            x: nums[0] * 2,
            y: nums[1] * 2,
        };

        let mut is_location_valid = false;
        let mut relation = (0, 0, 0, 0);
        if centers[0].included(&p, distance) {
            is_location_valid = true;
            relation.0 = 1;
        }
        if centers[1].included(&p, distance) {
            is_location_valid = true;
            relation.1 = 1;
        }
        if centers[2].included(&p, distance) {
            is_location_valid = true;
            relation.2 = 1;
        }
        if centers[3].included(&p, distance) {
            is_location_valid = true;
            relation.3 = 1;
        }
        if is_location_valid {
            relations = relations.generate(relation);
        }
    }

    println!("{}", relations.result());
}
