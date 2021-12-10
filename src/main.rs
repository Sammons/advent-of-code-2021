use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
};

struct Input {
    grid: Vec<Vec<i8>>,
}

impl Input {
    fn from_file(path: &str) -> Result<Input, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let grid = file_str
            .split("\n")
            .map(|line| {
                let vec: Vec<i8> = line
                    .trim()
                    .chars()
                    .map(|c| (c as i8 - '0' as i8) as i8)
                    .collect();
                vec
            })
            .collect();
        Ok(Input { grid })
    }
}

fn main_pt1() {
    let input_res = Input::from_file("./src/input.txt");
    match input_res {
        Ok(input) => {
            let mut sum: i32 = 0;
            for y in 0..input.grid.len() {
                for x in 0..input.grid[y].len() {
                    if is_low_point(&input, &x, &y) {
                        sum += 1 + input.grid[y][x] as i32;
                    }
                }
            }
            println!("{}", sum)
        }
        Err(e) => println!("{}", e),
    }
}

fn get_adjacencies(input: &Input, _x: &usize, _y: &usize) -> Vec<((usize, usize),i8)> {
    let x = *_x;
    let y = *_y;
    let get_y_x = |y, x| -> Option<i8> {
        input
            .grid
            .get(y)
            .map(|g: &Vec<i8>| g.get(x))
            .flatten()
            .map(|v: &i8| v.clone())
    };
    let mut adjacent_neighbors: Vec<((usize, usize),i8)> = vec![];
    let adjacencies = [
        y.checked_add(1).map(|v| ((x, v), get_y_x(v, x))),
        y.checked_sub(1).map(|v| ((x, v), get_y_x(v, x))),
        x.checked_add(1).map(|v| ((v, y), get_y_x(y, v))),
        x.checked_sub(1).map(|v| ((v, y), get_y_x(y, v))),
    ];
    for adj in adjacencies {
      if let Some((coords,Some(height))) = adj {
        adjacent_neighbors.push((coords, height));
      }
    }
    adjacent_neighbors
}

fn is_low_point(input: &Input, _x: &usize, _y: &usize) -> bool {
    let x = *_x;
    let y = *_y;
    let cur_value = input.grid[y][x];
    let adjacencies = get_adjacencies(input, _x, _y);
    for (_, height) in adjacencies {
        if height <= cur_value {
            return false;
        }
    }
    return true;
}

struct Basin {
    id: i32,
    size: usize,
}
struct BasinWatcher {
    basins: HashMap<i32, usize>,
    basin_members: Vec<Vec<Option<i32>>>,
}

impl BasinWatcher {
    fn from_input(input: &Input) -> BasinWatcher {
        let y_len = input.grid.len();
        let x_len = input.grid[0].len();
        let basin_members = (0..y_len)
            .map(|_| {
                return (0..x_len)
                    .map(|_| None as Option<i32>)
                    .collect::<Vec<Option<i32>>>();
            })
            .collect();
        BasinWatcher {
            basin_members,
            basins: HashMap::new(),
        }
    }
    fn add_member_to_basin(&mut self, basin_id: i32, x: usize, y: usize) {
      self.basin_members[y][x] = Some(basin_id);
      if self.basins.contains_key(&basin_id) {
        let v = self.basins.get_mut(&basin_id).unwrap();
        *v += 1;
      } else {
        self.basins.insert(basin_id, 1);
      }
    }
    fn pour_paint(&mut self, input: &Input, x: usize, y: usize) {
      let neighbors = get_adjacencies(input, &x, &y);
      // assume current x,y is painted since we start with sinks
      let cur_basin = self.basin_members[y][x].unwrap();
      let cur_height = input.grid[y][x];
      for ((nx,ny), height) in neighbors {
        if height >= cur_height && height != 9 && self.basin_members[ny][nx].is_none() {
          self.add_member_to_basin(cur_basin, nx, ny);
          self.pour_paint(input, nx, ny);
        }
      }
    }
    fn detect_basins(&mut self, input: &Input) -> Vec<(&i32, &usize)> {
        let mut sources: Vec<(usize, usize)> = vec![];
        let mut basin_ids = 0;
        for y in 0..input.grid.len() {
            for x in 0..input.grid[y].len() {
                if is_low_point(&input, &x, &y) {
                    sources.push((x, y));
                    basin_ids += 1;
                    self.add_member_to_basin(basin_ids, x, y);
                }
            }
        }
        println!("{} sources detected", sources.len());
        for (x, y) in sources {
            self.pour_paint(input, x, y);
        }
        self.basins.iter().map(|(k,v)| (k,v)).collect()
    }
}

fn main() {
    let input_res = Input::from_file("./src/input.txt");
    match input_res {
        Ok(input) => {
            let mut bw = BasinWatcher::from_input(&input);
            let mut basins = bw.detect_basins(&input);
            basins.sort_by(|b,a| a.1.cmp(b.1));
            let mut mult = 1;
            for b in basins.iter().take(3) {
              println!("{:?}", b);
              mult *= b.1;
            }
            println!("{:?}", mult);
        }
        Err(e) => println!("{}", e),
    }
}
