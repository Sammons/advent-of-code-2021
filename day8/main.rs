use std::{collections::HashSet, borrow::BorrowMut};

struct Line {
    samples: Vec<String>,
    ciphers: Vec<String>,
}
struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn from_file(path: &str) -> Result<Input, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let line_by_line = file_str.split('\n');
        let lines = line_by_line
            .map(|file_line| {
                let left_right_split: Vec<&str> = file_line.split('|').collect();
                let left = left_right_split[0];
                let right = left_right_split[1];
                let mut samples: Vec<String> = left
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| {
                      let mut v = vec![];
                      for c in s.chars() {
                        v.push(c);
                      }
                      v.sort();
                      v.iter().collect()
                    })
                    .collect();
                samples.sort_by(|a,b| a.len().cmp(&b.len()));
                let ciphers = right
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                Line { samples, ciphers }
            })
            .collect();
        Ok(Input { lines })
    }
}

struct Combo {
    seq: Vec<char>,
    remaining: Vec<char>,
}

fn all_combinations_of_a(chars: Vec<char>) -> Vec<Vec<char>> {
    let mut combos: Vec<String> = vec!["".to_string()];
    let mut observed = HashSet::new();
    while combos.len() > 0 {
        match combos.pop() {
            None => (),
            Some(combo) => {
                for c in &chars {
                    let new_s = combo.to_string() + &c.to_string();
                    if !observed.contains(&new_s) {
                        if new_s.len() < chars.len() {
                            combos.push(new_s.clone());
                        }
                        observed.insert(new_s);
                    }
                }
            }
        }
    }
    observed
        .iter()
        .filter(|v| v.len() == chars.len())
        .map(|s| s.chars().collect())
        .collect()
}

fn all_combinations_of_b(chars: &Vec<char>) -> Vec<Vec<char>> {
    let mut combos: Vec<Vec<char>> = vec![];
    for c in chars {
        let mut remaining = chars.clone();
        remaining.retain(|c2| c2 != c);
        for combo in all_combinations_of_b(&remaining) {
            combos.push([[*c].to_vec(), combo].concat());
        }
    }
    if combos.len() > 0 {
        combos
    } else {
        vec![vec![]]
    }
}

fn main_pt1() {
    let input_res = Input::from_file("./src/input.txt");
    match input_res {
        Ok(input) => {
            let mut count = 0;
            for line in input.lines {
                for word in line.ciphers {
                    if [2, 3, 4, 7].contains(&word.len()) {
                        count += 1;
                    }
                }
            }
            println!("{}", count);
        }
        Err(e) => println!("{}", e),
    }
}

fn try_decode(
  nums: &[String; 10],
  ciphers: &Vec<String>,
  key: &Vec<char>
) -> Option<Vec<usize>> {
  let n: Vec<char> ="abcdefg".chars().collect();
  let mut decoded_values = vec![];
  for word in ciphers {
    let mut decoded_chars: Vec<char> = word.chars()
      .map(|c| {
        for i in 0..n.len() {
          if key[i] == c {
            return n[i];
          }
        }
        panic!("invalid key");
      }).collect();
    decoded_chars.sort();
    let decoded: String = decoded_chars.iter().collect();
    for (idx, n) in nums.iter().enumerate() {
      if *n == decoded {
        decoded_values.push(idx);
      }
    }
  }
  if decoded_values.len() == ciphers.len() {
    return Some(decoded_values);
  } else {
    None
  }
}

fn main() {
    let nums = [
        "abcefg".to_string(),
        "cf".to_string(),
        "acdeg".to_string(),
        "acdfg".to_string(),
        "bcdf".to_string(),
        "abdfg".to_string(),
        "abdefg".to_string(),
        "acf".to_string(),
        "abcdefg".to_string(),
        "abcdfg".to_string(),
    ];
    let combos = all_combinations_of_b(&['a', 'b', 'c', 'd', 'e', 'f', 'g'].to_vec());
    println!("combo len {}", combos.len());
    let input_res = Input::from_file("./src/input.txt");
    let combos = all_combinations_of_b(&"abcdefg".chars().collect());
    match input_res {
        Ok(input) => {
            let mut count = 0;
            for line in input.lines.iter() {
              println!("Evaluating {}", line.ciphers.join(" "));
              for key in &combos {
                match try_decode(&nums, &line.samples, key) {
                  Some(_) => {
                    let decoded_values_digits = try_decode(&nums, &line.ciphers, key);
                    let numeric = decoded_values_digits.iter()
                      .flatten()
                      .map(|d| d.to_string())
                      .collect::<String>();
                    match numeric.parse::<i32>() {
                      Ok(v) => count += v,
                      Err(e) => {
                        println!("{}", e.to_string());
                      }
                    }
                  },
                  None => ()
                }
              }
            }
            println!("{}", count);
        }
        Err(e) => println!("{}", e),
    }
}
