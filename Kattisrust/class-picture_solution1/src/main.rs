use std::io;

fn run(people_names: &Vec<String>, relationships: &Vec<Vec<bool>>) {
    let mut results = Vec::new();

    let mut stop_sign = false;
    generate(
        &mut results,
        vec![],
        relationships.len() as u16,
        &relationships,
        &mut stop_sign,
    );

    if results.len() == 0 {
        println!("You all need therapy.");
    } else {
        let k: Vec<String> = results[0]
            .iter()
            .map(|&x| people_names[x as usize].clone())
            .collect();
        println!("{}", k.join(" "));
    }
}

fn generate(
    results: &mut Vec<Vec<u8>>,
    combination: Vec<u8>,
    left_counts: u16,
    relationships: &Vec<Vec<bool>>,
    stop_sign: &mut bool,
) {
    if *stop_sign {
        return;
    }
    if left_counts == 0 {
        results.push(combination);
        *stop_sign = true;
        return;
    }
    let whole_counts = relationships.len();

    if combination.len() == 0 {
        for i in 0..whole_counts {
            generate(
                results,
                vec![i as u8],
                left_counts - 1,
                &relationships,
                stop_sign,
            );
        }
    } else {
        let last = combination[combination.len() - 1];
        for i in 0..whole_counts {
            if not_in_combination(i as u8, &combination) && relationships[i][last as usize] {
                let mut k = combination.clone();
                k.push(i as u8);
                generate(results, k, left_counts - 1, &relationships, stop_sign);
            }
        }
    }
}

fn not_in_combination(i: u8, combination: &Vec<u8>) -> bool {
    for &item in combination {
        if item == i {
            return false;
        }
    }
    true
}

fn find_index(people_names: &Vec<String>, name: &String) -> usize {
    let mut x = 0;
    for i in 0..people_names.len() {
        if &people_names[i] == name {
            x = i;
        }
    }
    x
}

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let mut counts;
        match buffer.trim().parse::<usize>() {
            Ok(x) => counts = x,
            Err(_) => break,
        }

        let mut people_names = Vec::with_capacity(counts);

        for _ in 0..counts {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            people_names.push(String::from(buffer.trim()));
        }
        people_names.sort();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let pair_counts = buffer.trim().parse::<usize>().unwrap();

        let mut relationships = vec![vec![true; counts]; counts]; // false means bad
        for _ in 0..pair_counts {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            let pair: Vec<String> = buffer.split_whitespace().map(|s| s.to_string()).collect();
            let idx1 = find_index(&people_names, &pair[0]);
            let idx2 = find_index(&people_names, &pair[1]);
            relationships[idx1][idx2] = false;
            relationships[idx2][idx1] = false;
        }

        run(&people_names, &relationships);
    }
}
