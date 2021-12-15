use std::{
    collections::{HashMap, HashSet},
    iter::FromFn,
    ops::DerefMut,
};

struct Problem {
    grid: Vec<Vec<u8>>,
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

fn adjacent_coords(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut adjacent_neighbors: Vec<(usize, usize)> = vec![];
    let width = grid[0].len();
    let height = grid.len();
    let adjacencies = [
        bounded_tuple_add((x, y), (1, 0), (width, height)),
        bounded_tuple_add((x, y), (-1, 0), (width, height)),
        bounded_tuple_add((x, y), (0, 1), (width, height)),
        bounded_tuple_add((x, y), (0, -1), (width, height)),
    ];
    for adj in adjacencies {
        if adj.is_some() {
            adjacent_neighbors.push(adj.unwrap())
        }
    }
    adjacent_neighbors
}

struct Path {
    // nodes: HashSet<(usize, usize)>,
    risk: usize,
    cur_coord: (usize, usize),
}

impl Problem {
    fn from_file(path: &str) -> Result<Problem, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let lines: Vec<String> = file_str
            .split("\n")
            .map(|line| line.trim().to_string())
            .collect();
        let mut grid = vec![];
        for dy in 0..5 {
            for y in 0..lines.len() {
                let line = &lines[y];
                let mut row = vec![];
                for dx in 0..5 {
                    for char in line.chars() {
                        let v: u8 = char.to_string().parse::<u8>().unwrap() + dx as u8 + dy as u8;
                        if v > 9 {
                            row.push(v - 9);
                        } else {
                            row.push(v);
                        }
                    }
                }
                grid.push(row);
            }
        }
        Ok(Problem { grid })
    }
    fn print(&self) {
        println!("");
    }

    fn traverse_to_result(
        &self,
        start_coord: (usize, usize),
        end_coord: (usize, usize),
    ) -> Option<usize> {
        let mut start_list = HashSet::new();
        start_list.insert(start_coord);
        let mut queue = vec![Path {
            cur_coord: start_coord,
            risk: 0
            // nodes: start_list,
        }];
        let mut best_path_to_coord: HashMap<(usize, usize), usize> = HashMap::new();
        while queue.len() > 0 {

            let next = queue.pop().unwrap();
            let (x, y) = next.cur_coord;
            let mut neighbors = adjacent_coords(&self.grid, x, y);
            neighbors.sort_by(|(x2, y2), (x1, y1)| {
                let a = (x2 + y2) as i64;
                let b = (x1 + y1) as i64;
                a.cmp(&b)
            });
            // println!("{:?}", neighbors);
            for n in neighbors {
                let (nx, ny) = n;
                let new_risk = next.risk + self.grid[ny][nx] as usize;
                if let Some(best) = best_path_to_coord.get(&n) {
                    if best <= &new_risk {
                        continue;
                    }
                }
                if let Some(best) = best_path_to_coord.get(&end_coord) {
                    if best <= &new_risk {
                        continue;
                    }
                }
                let new_path = Path {
                    risk: new_risk,
                    cur_coord: n,
                };
                best_path_to_coord.insert(n, new_risk);
                if n == end_coord {
                    // println!("{}", new_risk);
                    continue;
                }
                queue.push(new_path)
            }
        }
        best_path_to_coord.get(&end_coord).map(|v| v.to_owned())
    }
}

fn main() {
    let input_res = Problem::from_file("./src/input.txt");
    match input_res {
        Ok(mut input) => {
            let risk =
                input.traverse_to_result((0, 0), (input.grid[0].len() - 1, input.grid.len() - 1));
            println!("{:?}", risk);
            // for y in 0..input.grid.len() {
            //     for x in 0..input.grid[y].len() {
            //         if path_members.contains(&(x, y)) {
            //             print!("{}", "*")
            //         } else {
            //             print!("{}", input.grid[y][x]);
            //         }
            //     }
            //     print!("\n");
            // }
            // println!("Done {:?} {:?}", path_members);
        }
        Err(e) => println!("{}", e),
    }
}
