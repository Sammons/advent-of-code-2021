
fn run() -> Result<(), String> {
  let str = std::fs::read_to_string("./src/input.txt").map_err(|e| e.to_string())?;
  let nums: Vec<u32> = str.split_ascii_whitespace().filter_map(|s| s.parse::<u32>().ok()).collect();
  let mut count = 0;
  let mut prev_sum = 0;
  prev_sum = nums[0] + nums[1] + nums[2];
  for idx in 3 .. nums.len() {
      let cur_sum = prev_sum - nums[idx - 3] + nums[idx];
      if cur_sum > prev_sum {
        count += 1;
      }
      prev_sum = cur_sum;
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
