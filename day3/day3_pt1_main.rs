
fn run() -> Result<(), String> {
  let str = std::fs::read_to_string("./src/input.txt").map_err(|e| e.to_string())?;
  let lines: Vec<&str> = str.split("\n").map(|l| l.trim()).collect();
  let width = lines[0].len();
  let mut one_values = vec![0; width];
  let mut zero_values = vec![0; width];
  let _: Vec<()> = lines.iter().flat_map(|l: &&str| -> Option<()> {
    for (idx, char) in l.char_indices() {
      match char {
        '1' => one_values[idx] += 1,
        '0' => zero_values[idx] += 1,
        _ => ()
      };
    }
    return None;
  }).collect();
  let mut gamma: u64 = 0;
  let mut epsilon: u64 = 0;
  for idx in 0..width {
    let (most_common_bit, least_common_bit) = 
      if one_values[idx] == 0 {
        (0, 0)
      } else if zero_values[idx] == 0 {
        (1, 1)
      } else if one_values[idx] < zero_values[idx] {
        (0, 1)
      } else {
        (1, 0)
      };
    println!("{},{}", most_common_bit, least_common_bit);
    gamma = gamma << 1;
    gamma = gamma | most_common_bit;
    epsilon = epsilon << 1;
    epsilon = epsilon | least_common_bit;
   }
  println!("Done! x {} y {} mult {}", gamma, epsilon, gamma * epsilon);
  Ok(())
}

fn main() {
  match run() {
      Err(e) => println!("Failed {}", e),
      Ok(_) => ()
  }
}
