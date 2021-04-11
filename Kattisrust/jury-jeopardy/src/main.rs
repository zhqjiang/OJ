use std::io::{self, BufRead};

fn run(s: &String) {
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut d: usize = 0;

    let mut top_boundry: i16 = 101;
    let mut bottom_boundry: i16 = 99;
    let mut right_boundry: i16 = 1;

    let mut maze = [[false; 200]; 100];

    let mut x: i16 = 0;
    let mut y: i16 = 100;
    maze[0][100] = true;

    for &i in s.as_bytes() {
        match i as char {
            'F' => {}
            'R' => d = (d + 1) % 4,
            'L' => d = (d + 3) % 4,
            'B' => d = (d + 2) % 4,
            _ => {}
        }

        y = y + directions[d].1;
        x = x + directions[d].0;
        if x >= right_boundry {
            right_boundry += 1;
        }
        if y >= top_boundry {
            top_boundry += 1;
        }
        if y <= bottom_boundry {
            bottom_boundry -= 1;
        }
        maze[x as usize][y as usize] = true;
    }
    println!("{} {}", top_boundry - bottom_boundry + 1, right_boundry + 1);

    for y in (bottom_boundry..(top_boundry + 1)).rev() {
        for x in 0..right_boundry + 1 {
            let symbol = if maze[x as usize][y as usize] {
                '.'
            } else {
                '#'
            };
            print!("{}", symbol);
        }
        print!("\n");
    }
}

fn main() {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(_) => {}
    }

    let mut vec = Vec::new();
    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        vec.push(line);
    }

    print!("{}", buffer);
    for line in vec {
        run(&line);
    }
}
