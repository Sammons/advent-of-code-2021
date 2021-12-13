use std::{
    collections::{HashMap, HashSet},
};

struct Input {
    pub from_to_links: HashMap<String, HashSet<String>>
}

impl Input {
    fn from_file(path: &str) -> Result<Input, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let lines: Vec<String> = file_str
            .split("\n")
            .map(|line| {
                line.trim().to_string()
            }).collect();
        let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
        for line in lines {
            let pieces: Vec<&str> = line.split('-').collect();
            let from = pieces[0];
            let to = pieces[1];
            if !adjacency.contains_key(from) {
                adjacency.insert(from.to_string(), HashSet::new());
            }
            if !adjacency.contains_key(to) {
                adjacency.insert(to.to_string(), HashSet::new());
            }
            adjacency.get_mut(from).unwrap().insert(to.to_string());
            adjacency.get_mut(to).unwrap().insert(from.to_string());
        }
        Ok(Input {
            from_to_links: adjacency
        })
    }
}



fn main() {
    let input_res = Input::from_file("./src/input.txt");
    match input_res {
        Ok(input) => {
          let mut paths: Vec<(bool, Vec<String>)> = vec![(false, vec!["start".to_string()])];
          let mut complete: Vec<Vec<String>> = vec![];
          while paths.len() > 0 {
              let (dup_present, next) = paths.pop().unwrap();
              let last_cave = next.last().unwrap();
              if last_cave == "end" {
                  complete.push(next);
                  continue;
              }
              if let Some(children) = input.from_to_links.get(last_cave) {
                  for child in children {
                      if child == "start" {
                          continue;
                      }
                      // if lowercase skip if already contained
                      let mut new_duplicate = dup_present;
                      if &child.to_lowercase() == child {
                        if next.contains(child) {
                            if dup_present {
                                continue;
                            } else {
                                new_duplicate = true;
                            }
                        }
                      }
                      let mut new_path = next.clone();
                      new_path.push(child.to_string());
                      paths.push((new_duplicate, new_path));
                  }
              } else {
                  println!("no children for {}", last_cave);
              }
          }
          println!("path count {}", complete.len());

        }
        Err(e) => println!("{}", e),
    }
}
