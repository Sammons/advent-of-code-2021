use std::{
  borrow::BorrowMut,
  collections::{HashMap, HashSet},
};

struct Input {
  lines: Vec<Vec<char>>,
}

impl Input {
  fn from_file(path: &str) -> Result<Input, String> {
      let res = std::fs::read_to_string(path);
      let file_str = res.map_err(|e| e.to_string())?;
      let lines = file_str
          .split("\n")
          .map(|line| {
              let vec: Vec<char> = line
                  .trim()
                  .chars()
                  .collect();
              vec
          })
          .collect();
      Ok(Input { lines })
  }
}

fn main() {
  let input_res = Input::from_file("./src/input.txt");
  match input_res {
      Ok(input) => {
        let mut corruptions = vec![0; 4];
        let open_symbols = [
          '{',
          '(',
          '[',
          '<'
        ];
        let autocorrect_score_values = [
              3,
              1,
              2,
              4
            ];
        let close_symbols = [
          '}',
          ')',
          ']',
          '>'
        ];
        let scores = [
          1197,
          3,
          57,
          25137
        ];
        let mut autocorrect_scores: Vec<usize> = vec![];
          for line in input.lines {
            let mut stack = vec![];
            let mut find_corruption = || {
              for _c in &line {
                let c = *_c;
                for i in 0..4 {
                  if c == open_symbols[i] {
                    stack.push(c);
                    continue;
                  }
                  if c == close_symbols[i] {
                    let open_sym_idx = open_symbols
                      .iter()
                      .enumerate()
                      .find_map(|(idx, open_c)| {
                        if open_c == stack.last().unwrap() {
                          Some(idx)
                        } else {
                          None
                        }
                      }).unwrap();
                    if stack.len() == 0 || i != open_sym_idx {
                      // println!("Expected {} but found {} instead", close_symbols[open_sym_idx], c);
                      corruptions[i] += 1;
                      return true;
                    } else {
                      stack.pop();
                    }
                  }
                }
              }
              return false;
            };
            if find_corruption() {
              continue;
            };
            let mut autocorrections: Vec<char> = vec![];
            while stack.len() > 0 {
              for i in 0..4 {
                if open_symbols[i] == *stack.last().unwrap() {
                  autocorrections.push(close_symbols[i]);
                  stack.pop();
                  break;
                }
              }
            }
            let mut autocorrect_score = 0;
            for correction in &autocorrections {
              let mut score = || {
                for i in 0..4 {
                  if *correction == close_symbols[i] {
                    autocorrect_score *= 5;
                    autocorrect_score += autocorrect_score_values[i];
                    return;
                  }
                }
              };
              score();
            }
            // println!("{:?} -> {}", autocorrections, autocorrect_score);
            autocorrect_scores.push(autocorrect_score);
            // println!("autocorrected with {:?}", autocorrections);
          }
          autocorrect_scores.sort();
          let middle = autocorrect_scores[autocorrect_scores.len()/2];
          let mut sum = 0;
          println!("{:?}", corruptions);
          for i in 0..4 {
            sum += corruptions[i] * scores[i];
          }
          println!("pt1 -> {}", sum);
          println!("pt2 -> {}", middle);
      }
      Err(e) => println!("{}", e),
  }
}
