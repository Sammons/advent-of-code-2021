use std::{collections::HashMap};

struct BingoTile {
    pub selected: bool,
    value: u32,
    x: usize,
    y: usize,
}

enum SelectOutcome {
    Bingo,
    NotBingo,
}

impl<'a> BingoTile {
    fn new(value: u32, x: usize, y: usize) -> BingoTile {
        BingoTile {
            selected: false,
            value,
            x,
            y,
        }
    }

    fn mark_selected(&'a mut self) {
        self.selected = true;
    }
}

struct BingoBoard {
    matrix: Vec<Vec<BingoTile>>,
    tiles: HashMap<u32, (usize,usize)>,
}

impl<'a> BingoBoard {
    fn mark_tile(&mut self, value: &u32) -> Option<(usize, usize)> {
        let tile_option = self.tiles.get(value);
        match tile_option {
            Some((x,y)) => {
                self.matrix[*x][*y].mark_selected();
                Some((*x,*y))
            }
            None => None
        }
    }
    fn is_tile_in_bingo(&mut self, (x,y): (&usize, &usize)) -> bool {
        let tile = &self.matrix[*x][*y];
        let width = self.matrix.len();
        let mut top_left_to_bottom_right = (0..width).zip(0..width);
        // let mut top_right_to_bottom_left = (0..width).rev().zip(0..width);
        let mut left_to_right_x = 0..width;
        let mut top_to_bottom_y = 0..width;
        let is_tile_part_of_bingo = top_left_to_bottom_right.all(|(x, y)| self.matrix[x][y].selected)
            // || top_right_to_bottom_left.all(|(x, y)| self.matrix[x][y].selected)
            || left_to_right_x.all(|x| self.matrix[x][tile.y].selected)
            || top_to_bottom_y.all(|y| self.matrix[tile.x][y].selected);
        if is_tile_part_of_bingo {
            println!("Bingo! {}", tile.value);
            let w = self.matrix.len();
            for y in 0..w {
              for x in 0..w {
                    print!("{}:{} ", self.matrix[x][y].selected, self.matrix[x][y].value);
              }
              print!("\n");
            }
        }
        is_tile_part_of_bingo
    }
    fn tally_unmarked(&self) -> u32 {
        self.tiles.iter().filter_map(|(_, (x,y))| {
            let t = &self.matrix[*x][*y];
            if t.selected {
                None
            } else {
                Some(t.value)
            }
        }).sum()
    }
}

struct InputStructure {
    called_numbers: Vec<u32>,
    bingo_boards: Vec<BingoBoard>,
}

impl<'a> InputStructure {
    fn parse_csv_u32s(str: &str, delim: char) -> Vec<u32> {
        str.split(delim).flat_map(|n| n.parse::<u32>().ok()).collect()
    }

    fn parse_line_bundle_into_board(vec: Vec<&&str>) -> BingoBoard {
        let lines_as_numbers = vec.iter()
            .map(|s| InputStructure::parse_csv_u32s(s, ' '));
        let mut bingo_tiles_by_num: HashMap<u32, (usize, usize)> = HashMap::new();
        let mut bingo_tiles_by_coord: Vec<Vec<BingoTile>> = vec![];
        for (y, col) in lines_as_numbers.enumerate() {
            for (x, num) in col.iter().enumerate() {
                if x + 1 > bingo_tiles_by_coord.len() {
                    bingo_tiles_by_coord.push(Vec::new());
                }
                bingo_tiles_by_coord[x].push(BingoTile::new(*num, x, y));
                bingo_tiles_by_num.insert(*num, (x,y));
            }
        }
        BingoBoard { matrix: bingo_tiles_by_coord, tiles: bingo_tiles_by_num }
    }

    fn parse_lines(file_content: &String) -> InputStructure {
        let lines: Vec<&str> = file_content.split('\n').collect();
        let called_numbers = InputStructure::parse_csv_u32s(&lines[0], ',');
        let mut bingo_boards = vec![];

        let mut line_idx = 0usize;
        while line_idx < lines.len() {
            let mut current_board_lines: Vec<&&str> = vec![];
            loop {
                line_idx += 1;
                if line_idx < lines.len() && lines[line_idx].trim().len() > 0 {
                  current_board_lines.push(&lines[line_idx]);
                } else {
                  break;
                }
            }
            bingo_boards.push(InputStructure::parse_line_bundle_into_board(current_board_lines));
        }
        
        InputStructure {
            bingo_boards,
            called_numbers,
        }
    }

    
}
fn mark_tiles_in_boards(mut bingo_boards: Vec<&mut BingoBoard>, num: &u32) -> Vec<(usize, (usize, usize))> {
    (0..bingo_boards.len()).flat_map(|idx| {
        bingo_boards[idx].mark_tile(num).map(|t| (idx, t))
    }).collect()
}

fn run() -> Result<(), String> {
    let read_result = std::fs::read_to_string("./src/input.txt").map_err(|e| e.to_string())?;
    let input =  InputStructure::parse_lines(&read_result);
    let mut boards = input.bingo_boards;
    println!("Numbers to call {}", input.called_numbers.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
    let bingo_board = input.called_numbers.iter().find_map(|num| {
        println!("Calling {}", num);
        let bingos = mark_tiles_in_boards(boards.iter_mut().collect(), num);
        bingos.iter().find_map(|(board_idx, (x,y))| {
            if boards[*board_idx].is_tile_in_bingo((x,y)) {
                Some(boards[*board_idx].tally_unmarked() * num)
            } else {
                None
            }
        })
    });

    println!("Done! x {:?}", bingo_board);
    Ok(())
}


fn main() {
    match run() {
        Err(e) => println!("Failed {}", e),
        Ok(_) => (),
    }
}
