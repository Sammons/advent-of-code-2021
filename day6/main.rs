use std::{collections::HashMap};

fn main() {
    let mut sample = vec![
        3, 4, 3, 1, 2
    ];
    let mut input = vec![
        4, 5, 3, 2, 3, 3, 2, 4, 2, 1, 2, 4, 5, 2, 2, 2, 4, 1, 1, 1, 5, 1, 1, 2, 5, 2, 1, 1, 4, 4,
        5, 5, 1, 2, 1, 1, 5, 3, 5, 2, 4, 3, 2, 4, 5, 3, 2, 1, 4, 1, 3, 1, 2, 4, 1, 1, 4, 1, 4, 2,
        5, 1, 4, 3, 5, 2, 4, 5, 4, 2, 2, 5, 1, 1, 2, 4, 1, 4, 4, 1, 1, 3, 1, 2, 3, 2, 5, 5, 1, 1,
        5, 2, 4, 2, 2, 4, 1, 1, 1, 4, 2, 2, 3, 1, 2, 4, 5, 4, 5, 4, 2, 3, 1, 4, 1, 3, 1, 2, 3, 3,
        2, 4, 3, 3, 3, 1, 4, 2, 3, 4, 2, 1, 5, 4, 2, 4, 4, 3, 2, 1, 5, 3, 1, 4, 1, 1, 5, 4, 2, 4,
        2, 2, 4, 4, 4, 1, 4, 2, 4, 1, 1, 3, 5, 1, 5, 5, 1, 3, 2, 2, 3, 5, 3, 1, 1, 4, 4, 1, 3, 3,
        3, 5, 1, 1, 2, 5, 5, 5, 2, 4, 1, 5, 1, 2, 1, 1, 1, 4, 3, 1, 5, 2, 3, 1, 3, 1, 4, 1, 3, 5,
        4, 5, 1, 3, 4, 2, 1, 5, 1, 3, 4, 5, 5, 2, 1, 2, 1, 1, 1, 4, 3, 1, 4, 2, 3, 1, 3, 5, 1, 4,
        5, 3, 1, 3, 3, 2, 2, 1, 5, 5, 4, 3, 2, 1, 5, 1, 3, 1, 3, 5, 1, 1, 2, 1, 1, 1, 5, 2, 1, 1,
        3, 2, 1, 5, 5, 5, 1, 1, 5, 1, 4, 1, 5, 4, 2, 4, 5, 2, 4, 3, 2, 5, 4, 1, 1, 2, 4, 3, 2, 1,
    ];
    let mut runwith = input;
    let mut pre_calculations: HashMap<u64, u64> = HashMap::new();
    fn calculate_single_fish(v: u64, n: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
        if memo.contains_key(&(v, n)) {
            return *memo.get(&(v, n)).unwrap();
        }
        let mut count = 1;
        let mut timer = v;
        let mut remaining = n;
        while remaining > 0 {
          remaining -= 1;
          if timer == 0 {
            timer = 6;
            count += calculate_single_fish(8, remaining, memo);
          } else {
            timer -= 1;
          }
        }
        memo.insert((v, n), count);
        count
    }
    let n = 256;
    let mut memo = HashMap::new();
    let mut count: u64 = 0;
    for i in 0..9 {
        pre_calculations.insert(i, calculate_single_fish(i, n, &mut memo));
    }
    for idx in 0..runwith.len() {
        count += pre_calculations.get(&runwith[idx]).unwrap()
    }
    println!("Length {}", count);
}
