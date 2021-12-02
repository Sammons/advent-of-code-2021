
fn run() -> Result<(), String> {
  let str = std::fs::read_to_string("./src/input.txt").map_err(|e| e.to_string())?;
  let nums: Vec<u32> = str.split_ascii_whitespace().filter_map(|s| s.parse::<u32>().ok()).collect();
  let mut count = 0;
  for idx in 1 .. nums.len() {
      let prev = &nums[idx - 1];
      let cur = &nums[idx];
      if cur > prev {
          count = count + 1;
      }
  }
  println!("Done! {}", count);
  Ok(())
}

fn main() {
  match run() {
      Err(e) => println!("Failed {}", e),
      Ok(_) => ()
  }
}
