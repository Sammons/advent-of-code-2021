use std::{
    collections::{HashMap, HashSet},
    iter::FromFn,
    ops::{DerefMut, Range, RangeBounds},
};

struct Problem {
    target_area_min_x: i32,
    target_area_max_x: i32,
    target_area_min_y: i32,
    target_area_max_y: i32
}


impl Problem {
    fn print(&self) {
        println!("");
    }
    fn detect_result_pos(&self, x: &i32) -> i32 {
        (x * (x + 1)) / 2
    }
    fn detect_velocity_options_hitting_range(&self, range: Range<i32>) -> Vec<i32> {
        let mut options = vec![];
        for possibility in 1..range.end {
            let result_position = self.detect_result_pos(&possibility);
            if range.contains(&result_position) {
                options.push(possibility);
            }
        }
        options
    }
    fn detect_best_y_height_for_x(&self, x: &i32) -> i32 {
        let mut highest_pos = 0;
        let mut valid_y_values = vec![];
        for yv in 1..x*100 {
            let mut y = 0;
            let mut dv = yv;
            let mut cur_highest_pos = 0;
            let mut is_valid = false;
            while y >= self.target_area_min_y {
                y = y + dv;
                if y > cur_highest_pos {
                    cur_highest_pos = y
                }
                if y >= self.target_area_min_y && y <= self.target_area_max_y {
                    is_valid = true;
                    break;
                }
                dv -= 1;
            }
            if is_valid {
                valid_y_values.push(yv);
            }
            if is_valid && cur_highest_pos > highest_pos {
                highest_pos = cur_highest_pos;
            }
        }
        highest_pos
    }
    fn is_valid_combo(&self, x: &i32, y: &i32) -> bool {
        let mut xp = 0;
        let mut yp = 0;
        let mut dx = x.clone();
        let mut dy = y.clone();
        while xp < self.target_area_max_x + 1 && yp >= self.target_area_min_y {
            xp += dx;
            yp += dy;
            if dx > 0 {
                dx -= 1;
            }
            dy -= 1;
            if xp >= self.target_area_min_x && xp <= self.target_area_max_x &&
                yp >= self.target_area_min_y && yp <= self.target_area_max_y {
                    return true;
                }
        }
        false
    }
    fn best_height(&self) {
        let x_options = self.detect_velocity_options_hitting_range(
            self.target_area_min_x..self.target_area_max_x + 1
        );
        let min_x_option = x_options.iter().min().unwrap();
        let y_options: Vec<i32> = x_options.iter().map(|x| {
            self.detect_best_y_height_for_x(x)
        }).collect();
        println!("{:?}", y_options);
        
        // brute out all valid combos
        let mut count = 0;
        for x in 1..self.target_area_max_x + 1 {
            let final_x = self.detect_result_pos(&x);
            if final_x < self.target_area_min_x {
                continue;
            }
            for y in -600..600 {
                if self.is_valid_combo(&x, &y) {
                    count += 1
                }
            }
        };
        println!("{}", count);

    }
}

fn main() {
    let input_res = Problem {
        // x=57..116, y=-198..-148
        // not 4753
        target_area_min_x: 57,
        target_area_max_x: 116,
        target_area_min_y: -198,
        target_area_max_y: -148
        // sample x=20..30, y=-10..-5
        // 6,9 -> 45
        // 6 + 5 + 4 + 3 + 2 + 1
        // target_area_min_x: 20,
        // target_area_max_x: 30,
        // target_area_min_y: -10,
        // target_area_max_y: -5
    };
    input_res.best_height();
    // println!("{}", input_res.best_height());
}
