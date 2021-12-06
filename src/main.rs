use std::{collections::HashSet, hash::Hash, option::Iter};

struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn from_csv_pair(str: &str) -> Result<Point, String> {
        let pieces: Vec<&str> = str.split(',').collect();
        if pieces.len() != 2 {
            Err("Invalid point format".to_string())
        } else {
            Ok(Point {
                x: pieces[0].parse::<usize>().map_err(|e| e.to_string())?,
                y: pieces[1].parse::<usize>().map_err(|e| e.to_string())?,
            })
        }
    }
}
struct Line {
    points: Vec<Point>,
}
impl Line {
    fn from_file_str(str: &str) -> Result<Line, String> {
        let pieces: Vec<&str> = str.split(" -> ").collect();
        if pieces.len() != 2 {
            Err("Invalid line format".to_string())
        } else {
            Ok(Line {
                points: vec![
                    Point::from_csv_pair(pieces[0])?,
                    Point::from_csv_pair(pieces[1])?,
                ],
            })
        }
    }
    fn get_start_end_points(&self) -> (&Point, &Point) {
        (self.points.first().unwrap(), self.points.last().unwrap())
    }
    fn to_points(&self) -> Vec<Point> {
        let (a, b) = self.get_start_end_points();
        let delta = |a: usize, b: usize| {
            if a > b {
                a - b
            } else {
                b - a
            }
        };
        let x1 = a.x;
        let y1 = a.y;
        let x2 = b.x;
        let y2 = b.y;
        let x_range: Vec<usize> = if x1 == x2 {
            vec![x1; delta(y1, y2) + 1]
        } else if x1 < x2 {
            (x1..x2 + 1).collect()
        } else {
            (x2..x1 + 1).rev().collect()
        };
        let y_range: Vec<usize> = if y1 == y2 {
            vec![y1; delta(x1, x2) + 1]
        } else if y1 < y2 {
            (y1..y2 + 1).collect()
        } else {
            (y2..y1 + 1).rev().collect()
        };
        if y_range.len() != x_range.len() {
          panic!("Ranges do not match in length");
        }
        x_range
            .iter()
            .zip(y_range.iter())
            .map(|(x, y)| Point { x: *x, y: *y })
            .collect()
    }
}

struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn parse(file_contents: &str) -> Input {
        Input {
            lines: file_contents
                .split('\n')
                .flat_map(|file_line| Line::from_file_str(file_line).ok())
                .collect(),
        }
    }
}

struct ResizingCountGrid {
    dangerous_coords: HashSet<(usize, usize)>,
    counts: Vec<Vec<u32>>,
}

impl ResizingCountGrid {
    fn increment_point(&mut self, point: &Point) {
        if self.counts.len() < point.x + 1 {
            self.counts.resize(point.x + 1, vec![]);
        }
        if self.counts[point.x].len() < point.y + 1 {
            self.counts[point.x].resize(point.y + 1, 0);
        }
        self.counts[point.x][point.y] += 1;
        // println!(
        //     "Incrementing {},{} -> {}",
        //     point.x, point.y, self.counts[point.x][point.y]
        // );
        if self.counts[point.x][point.y] >= 2 {
            self.dangerous_coords.insert((point.x, point.y));
        }
    }
    fn increment_points_in_line(&mut self, line: &Line) {
        for point in line.to_points() {
            self.increment_point(&point)
        }
    }
}

fn main() -> Result<(), String> {
    let file_contents = std::fs::read_to_string("./src/input.txt").map_err(|e| e.to_string())?;
    let input = Input::parse(&file_contents);
    let mut grid = ResizingCountGrid {
        counts: vec![],
        dangerous_coords: HashSet::new(),
    };
    for line in input.lines {
        grid.increment_points_in_line(&line)
    }
    println!("Dangerous count {}", grid.dangerous_coords.len());
    Ok(())
}
