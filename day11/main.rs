use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
};

struct Input {
    grid: [[i8; 10]; 10],
    flashes: Vec<(usize, usize, usize)>,
    overcharged: HashSet<(usize, usize)>,
}

impl Input {
    fn from_file(path: &str) -> Result<Input, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let mut grid = [[0i8; 10]; 10];
        let lines: Vec<Vec<char>> = file_str
            .split("\n")
            .map(|line| {
                let vec: Vec<char> = line.trim().chars().collect();
                vec
            })
            .collect();
        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                grid[y][x] = lines[y][x].to_string().parse().ok().unwrap();
            }
        }
        Ok(Input {
            grid,
            flashes: vec![],
            overcharged: HashSet::new(),
        })
    }
}

fn add_i32_to_usize(a: usize, b: i32) -> Option<usize> {
    if b < 0 {
        let _b = (b * -1) as usize;
        if _b > a {
            None
        } else {
            Some(a - _b)
        }
    } else {
        let _b = b as usize;
        Some(a + _b)
    }
}

// inclusive to zero
fn bounded_tuple_add(
    a: (usize, usize),
    b: (i32, i32),
    exclusive_bound: (usize, usize),
) -> Option<(usize, usize)> {
    let x = add_i32_to_usize(a.0, b.0).filter(|v| v < &exclusive_bound.0);
    let y = add_i32_to_usize(a.1, b.1).filter(|v| v < &exclusive_bound.1);
    if x.is_some() && y.is_some() {
        Some((x.unwrap(), y.unwrap()))
    } else {
        None
    }
}

fn adjacent_coords(x: usize, y: usize) -> Vec<(usize, usize)> {
  let mut adjacent_neighbors: Vec<(usize, usize)> = vec![];
  let adjacencies = [
      bounded_tuple_add((x, y), (1, 0), (10, 10)),
      bounded_tuple_add((x, y), (-1, 0), (10, 10)),
      bounded_tuple_add((x, y), (0, 1), (10, 10)),
      bounded_tuple_add((x, y), (0, -1), (10, 10)),
      // diag
      bounded_tuple_add((x, y), (-1, -1), (10, 10)),
      bounded_tuple_add((x, y), (-1, 1), (10, 10)),
      bounded_tuple_add((x, y), (1, 1), (10, 10)),
      bounded_tuple_add((x, y), (1, -1), (10, 10)),
  ];
  for adj in adjacencies {
      if adj.is_some() {
          adjacent_neighbors.push(adj.unwrap())
      }
  }
  adjacent_neighbors
}

impl Input {
    fn print(&self) {
        let mut s = "".to_string();
        for y in 0..10 {
            let mut line = vec![];
            for x in 0..10 {
                line.push(self.grid[y][x].to_string())
            }
            s += &[line.join(""), "\n".to_string()].to_vec().concat();
        }
        println!("{}", s);
    }
    fn increment_coord(&mut self, x: usize, y: usize) {
        self.grid[y][x] += 1;
        if self.grid[y][x] > 9 {
            self.overcharged.insert((x, y));
        }
    }
    fn reset_coord(&mut self, x: usize, y: usize) {
        self.grid[y][x] = 0;
        let tup = (x, y);
        if self.overcharged.contains(&tup) {
            self.overcharged.remove(&tup);
        }
    }
    fn enumerate_coords(&self) -> Vec<(usize, usize)> {
        let mut coords = vec![];
        for y in 0..10 {
            for x in 0..10 {
                coords.push((x, y));
            }
        }
        coords
    }
   

    fn tick(&mut self, t: &usize) -> usize {
        for (x, y) in self.enumerate_coords() {
            self.increment_coord(x, y);
        }
        let mut observed_flashes: HashSet<(usize, usize)> = HashSet::new();
        while self.overcharged.len() > 0 {
          let flashing_tiles: Vec<(usize, usize)> = self.overcharged.drain().collect();
          for (x, y) in flashing_tiles {
            if observed_flashes.contains(&(x, y)) {
              println!("Already flashed once!");
              continue;
            } else {
              observed_flashes.insert((x, y));
            }
            self.flashes.push((x, y, t.clone()));
            self.reset_coord(x, y);
            for (nx, ny) in adjacent_coords(x, y) {
              if !observed_flashes.contains(&(nx, ny)) {
                self.increment_coord(nx, ny);
              }
            }
          }
        }
        observed_flashes.len()
    }
}

fn main() {
    let input_res = Input::from_file("./src/input.txt");
    match input_res {
        Ok(mut input) => {
          for t in 0..10000 {
            let new_flash_count = input.tick(&t);
            if new_flash_count == 100 {
              println!("observed 100 flashes at t={}", t + 1);
              input.print();
              return;
            }
            // println!("T={}", t + 1);
            // input.print();
          }
          input.print();
          println!("flashes -> {}", input.flashes.len());
        }
        Err(e) => println!("{}", e),
    }
}
