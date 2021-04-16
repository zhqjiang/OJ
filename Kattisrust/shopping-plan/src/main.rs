use std::cmp::min;
use std::collections::HashMap;
use std::io;

// static LARGE: u64 = <u64>::pow(2, 63);
static LARGE: u64 = 200 * 100000 * 100000 * 100000;

struct Item {
    name: String,
}
impl Item {
    fn new(name: String) -> Item {
        Item { name }
    }
}

struct Store {
    x: i32,
    y: i32,
    items: Vec<(Item, i32)>,
    index: usize,
}

struct Bitmask(u16);
impl Bitmask {
    fn len(&self) -> usize {
        self.into_iter()
            .fold(0, |acc, x| acc + if x > 0 { 1 } else { 0 })
    }
}
impl<'a> IntoIterator for &'a Bitmask {
    type Item = usize;
    type IntoIter = BitmaskIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        BitmaskIntoIter(self, 0)
    }
}
struct BitmaskIntoIter<'a>(&'a Bitmask, usize);
impl<'a> Iterator for BitmaskIntoIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < 16 {
            if ((1 << self.1) & self.0 .0) > 0 {
                self.1 += 1;
                return Some(self.1 - 1);
            } else {
                self.1 += 1;
                return self.next();
            }
        } else {
            None
        }
    }
}

fn all_in_one_store(perishables: &Vec<Item>, store: &Store) -> bool {
    let mut contains_all = true;
    for item in perishables {
        match store.items.iter().position(|x| x.0.name == item.name) {
            Some(_) => {}
            None => contains_all = false,
        }
    }
    contains_all
}

fn check_perishables<'a>(
    stores: &Vec<&'a Store>,
    perishables: &Vec<Item>,
) -> (Vec<&'a Store>, bool) {
    if perishables.len() == 0 {
        return (vec![], true);
    }
    let mut av = vec![];

    let mut is_store_exist = false;
    for &store in stores {
        if all_in_one_store(perishables, store) {
            is_store_exist = true;
            av.push(store);
        }
    }

    (av, is_store_exist)
}

fn cal_simple(
    wishlist: &Vec<Item>,
    price_of_gas: i32,
    stores: &Vec<&Store>,
    items_prices: &HashMap<String, i32>,
) -> u64 {
    // shopping fee
    let mut shopping_fee = 0;
    for item in wishlist {
        match items_prices.get(&item.name) {
            Some(&price) => {
                shopping_fee += price;
            }
            None => {
                // there is a item that is not available
                return LARGE;
            }
        }
    }

    // calculate the travel cost(gas cost)
    let positions = stores.iter().map(|&x| (x.x, x.y)).collect();
    let gas_cost = get_min_distance(positions, (0, 0), (0, 0)) * price_of_gas as f64;

    let sum = shopping_fee as f64 + gas_cost;

    let final_res = (sum * 100000.0 * 100000.0) as u64;
    final_res
}

fn distance(pos1: (i32, i32), pos2: (i32, i32)) -> f64 {
    (((pos1.0 - pos2.0) * (pos1.0 - pos2.0) + (pos1.1 - pos2.1) * (pos1.1 - pos2.1)) as f64).sqrt()
}

fn travese(unused: u16, start: usize, end: usize, matrix: &Vec<Vec<u64>>) -> u64 {
    let mut res = LARGE;
    let unused_mask = Bitmask(unused);
    if unused_mask.len() == 0 {
        return matrix[start][end];
    }

    for choosed in &unused_mask {
        res = min(
            res,
            matrix[choosed][start] + travese(unused & !(1 << choosed), choosed, end, matrix),
        );
    }

    res
}

fn get_min_distance(positions: Vec<(i32, i32)>, start: (i32, i32), end: (i32, i32)) -> f64 {
    let mut new_positions = positions.clone();
    new_positions.insert(positions.len(), end);
    new_positions.insert(0, start);

    let mut matrix: Vec<Vec<u64>> = vec![vec![0; positions.len() + 2]; positions.len() + 2];

    for (i, pos_i) in new_positions.iter().enumerate() {
        for (j, pos_j) in new_positions.iter().enumerate() {
            let distancce_f64 = (((pos_i.0 - pos_j.0) * (pos_i.0 - pos_j.0)
                + (pos_i.1 - pos_j.1) * (pos_i.1 - pos_j.1))
                as f64)
                .sqrt();
            let distance_u64_save = (distancce_f64 * 100000.0 * 100000.0) as u64;
            matrix[i][j] = distance_u64_save;
            matrix[j][i] = distance_u64_save;
        }
    }

    let mut unused: u16 = 0;
    for i in 1..positions.len() + 1 {
        unused = unused | (1 << i);
    }
    travese(unused, 0, positions.len() + 1, &matrix) as f64 / 100000.0 / 100000.0
}

fn cal_one_way_home(
    perishables: &Vec<Item>,
    last_store: &Store,
    wishlist: &Vec<Item>,
    price_of_gas: i32,
    stores: &Vec<&Store>,
    items_prices: &HashMap<String, i32>,
) -> u64 {
    // shopping fee
    let mut are_all_items_found = true;
    let mut shopping_fee = 0;
    for item in wishlist {
        match items_prices.get(&item.name) {
            Some(&price) => {
                shopping_fee += price;
            }
            None => {
                are_all_items_found = false;
            }
        }
    }

    if !are_all_items_found {
        return LARGE;
    }

    // perishable items fee
    let mut perishable_fee = 0;
    for to_buy in perishables {
        for a_item in &last_store.items {
            if to_buy.name == a_item.0.name {
                perishable_fee += a_item.1;
            }
        }
    }

    // last store to home gas cost
    let last_gas_cost = distance((0, 0), (last_store.x, last_store.y)) * price_of_gas as f64;

    let s = stores
        .iter()
        .filter(|&&x| x.index != last_store.index)
        .map(|&x| (x.x, x.y))
        .collect();

    // other gas cost
    let other_gas_cost =
        get_min_distance(s, (0, 0), (last_store.x, last_store.y)) * price_of_gas as f64;

    let sum = (shopping_fee + perishable_fee) as f64 + last_gas_cost + other_gas_cost;

    (sum * 100000.0 * 100000.0) as u64
}

fn calculate(
    price_of_gas: i32,
    stores: &Vec<&Store>,
    perishables: &Vec<Item>,
    wishlist: &Vec<Item>,
) -> u64 {
    let mut res = LARGE;

    let check_perishables_result = check_perishables(stores, perishables);
    if !check_perishables_result.1 {
        return LARGE;
    }
    let available_perishable_stores = check_perishables_result.0;

    let mut items_prices: HashMap<String, i32> = HashMap::new();
    for &s in stores {
        for item in &s.items {
            match items_prices.get(&item.0.name) {
                Some(&x) => {
                    if item.1 < x {
                        items_prices.insert((item.0.name).clone(), item.1);
                    }
                }
                None => {
                    items_prices.insert((item.0.name).clone(), item.1);
                }
            }
        }
    }

    if available_perishable_stores.len() == 0 {
        res = min(
            res,
            cal_simple(wishlist, price_of_gas, stores, &items_prices),
        );
    } else {
        for last_store in available_perishable_stores {
            res = min(
                res,
                cal_one_way_home(
                    perishables,
                    last_store,
                    wishlist,
                    price_of_gas,
                    stores,
                    &items_prices,
                ),
            );
        }
    }

    res
}

fn solve(
    num_stores: i32,
    price_of_gas: i32,
    stores: &Vec<Store>,
    perishables: &Vec<Item>,
    wishlist: &Vec<Item>,
    case_number: usize,
) {
    let mut sum: u64 = LARGE;
    for i in 1..<i32>::pow(2, num_stores as u32) {
        let mut new_stores: Vec<&Store> = vec![];
        for i in Bitmask(i as u16).into_iter() {
            // only 5 or less stores should be reached
            if Bitmask(i as u16).len() <= 5 {
                new_stores.push(&stores[i]);
            }
        }

        let res = calculate(price_of_gas, &new_stores, perishables, wishlist);
        sum = min(res, sum);
    }
    println!(
        "Case #{}: {:.9}",
        case_number + 1,
        (sum as f64) / 100000.0 / 100000.0
    );
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    let testcase_count = buffer.trim().parse::<usize>().unwrap();

    for case_number in 0..testcase_count {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed");
        let nums: Vec<i32> = buffer
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let num_stores = nums[1];
        let price_of_gas = nums[2];

        let mut buffer2 = String::new();
        io::stdin().read_line(&mut buffer2).expect("Failed");
        let tmp: Vec<String> = buffer2
            .split_whitespace()
            .map(|list| String::from(list))
            .collect();

        let mut perishables: Vec<Item> = vec![];
        let mut wishlist = vec![];
        for item in tmp {
            if item.ends_with("!") {
                let mut i = item.clone();
                i.drain(item.len() - 1..);
                perishables.push(Item::new(i.clone()));
            } else {
                wishlist.push(Item::new(item));
            }
        }

        let mut stores: Vec<Store> = vec![];
        for i in 0..num_stores {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed");
            let mut store_info: Vec<String> = buffer
                .split_whitespace()
                .map(|list| String::from(list))
                .collect();

            let x = store_info[0].parse::<i32>().unwrap();
            let y = store_info[1].parse::<i32>().unwrap();

            store_info.drain(..2);
            let items = store_info
                .iter()
                .map(|s| {
                    let x: Vec<&str> = s.split(':').collect();
                    let name: String = x[0].parse().unwrap();
                    let price: i32 = x[1].parse().unwrap();
                    return (Item::new(name), price);
                })
                .collect();
            stores.push(Store {
                x,
                y,
                items,
                index: i as usize,
            });
        }

        solve(
            num_stores,
            price_of_gas,
            &stores,
            &perishables,
            &wishlist,
            case_number,
        );
    }
}
