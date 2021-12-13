use std::{
    collections::{HashMap, HashSet},
    slice::SliceIndex,
};

struct Fold {
    dim: String,
    value: usize,
}
struct Paper {
    grid: Vec<Vec<bool>>,
    folds: Vec<Fold>,
}

impl Paper {
    fn from_file(path: &str) -> Result<Paper, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let lines: Vec<String> = file_str
            .split("\n")
            .map(|line| line.trim().to_string())
            .collect();
        // let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
        let mut parsing_folds = false;
        let mut coords: Vec<(usize, usize)> = vec![];
        let mut folds: Vec<Fold> = vec![];
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;
        for line in lines {
            if line.trim().len() == 0 {
                parsing_folds = true;
                continue;
            }
            if parsing_folds {
                let pieces: Vec<&str> = line
                    .trim()
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .split('=')
                    .collect();
                let dim = pieces[0].to_string();
                let value: usize = pieces[1].parse().unwrap();
                folds.push(Fold {
                    dim: dim,
                    value: value,
                })
            } else {
                let pieces: Vec<&str> = line.trim().split(',').collect();
                let x: usize = pieces[0].parse().unwrap();
                if x > max_x {
                    max_x = x;
                }
                let y: usize = pieces[1].parse().unwrap();
                if y > max_y {
                    max_y = y;
                }
                coords.push((x, y));
            }
        }
        let mut grid = vec![];
        for (x, y) in coords {
            grid.resize(max_y + 1, vec![]);
            grid[y].resize(max_x + 1, false);
            grid[y][x] = true;
        }
        for y in 0..max_y {
            grid[y].resize(max_x + 1, false);
        }
        folds.reverse();
        Ok(Paper { folds: folds, grid })
    }
}

impl Paper {
    fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                print!("{}", if self.grid[y][x] { "#" } else { "." });
            }
            print!(" <- {} \n", y);
        }
    }
    fn fold_x(&mut self, x: usize) {
        for y in 0 ..self.grid.len() {
            let left = (0..x).rev();
            let right = x+1 .. self.grid[y].len();
            for (xl, xr) in left.zip(right) {
                self.grid[y][xl] = self.grid[y][xl] || self.grid[y][xr];
            }
            self.grid[y].truncate(x);
        }
    }
    fn fold_y(&mut self, y: usize) {
        // y-1 .. 0 will be overlaid on y + 1 .. max(y)
        let top = (0 .. y).rev();
        let bot = y+1 .. self.grid.len();
        for (yt, yb) in top.zip(bot) {
            for x in 0..self.grid[yt].len() {
                self.grid[yt][x] = self.grid[yt][x] || self.grid[yb][x];
            }
        }
        self.grid.truncate(y);
    }
    fn fold_once(&mut self) -> bool {
        if self.folds.len() == 0 {
            return false;
        }
        let f = self.folds.pop().unwrap();
        if f.dim == "x" {
            self.fold_x(f.value);
        } else {
            self.fold_y(f.value);
        }
        return true;
    }
    fn count_dots(&self) -> usize {
        let mut count: usize = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                count += if self.grid[y][x] { 1 } else { 0 };
            }
        }
        count
    }
}

fn main() {
    let input_res = Paper::from_file("./src/input.txt");
    match input_res {
        Ok(mut input) => {
            while input.fold_once() {};
            input.print();
            println!("Done {}", input.count_dots());
        }
        Err(e) => println!("{}", e),
    }
}
