
fn run() -> Result<(), String> {
  let str = std::fs::read_to_string("./src/input.txt").map_err(|e| e.to_string())?;
  let mut x_pos = 0;
  let mut y_pos = 0;
  let mut aim = 0;
  let _: Vec<()> = str.split("\n").flat_map(|l: &str| -> Option<()> {
    let pieces: Vec<&str> = l.trim().split_ascii_whitespace().collect();
    if pieces.len() != 2 {
      return None;
    }
    let cmd = pieces[0];
    let value = pieces[1].parse::<i32>().ok()?;
    match cmd {
      "forward" => {
        x_pos += value;
        y_pos += aim * value;
      },
      "up" => aim -= value,
      "down" => aim += value,
      _ => ()
    };
    return None;
  }).collect();
  println!("Done! x {} y {} mult {}", x_pos, y_pos, x_pos * y_pos);
  Ok(())
}

fn main() {
  match run() {
      Err(e) => println!("Failed {}", e),
      Ok(_) => ()
  }
}
