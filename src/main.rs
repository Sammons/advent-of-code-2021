fn bin_to_num(v: &str) -> u64 {
    u64::from_str_radix(v, 2).unwrap() // accept panic for this data set
}

fn detect_counts(lines: &Vec<&str>, width: &usize) -> (Vec<i32>, Vec<i32>) {
    let mut one_values = vec![0; *width];
    let mut zero_values = vec![0; *width];
    let _: Vec<()> = lines
        .iter()
        .flat_map(|l: &&str| -> Option<()> {
            for (idx, char) in l.char_indices() {
                match char {
                    '1' => one_values[idx] += 1,
                    '0' => zero_values[idx] += 1,
                    _ => (),
                };
            }
            return None;
        })
        .collect();
    (zero_values, one_values)
}

fn bit_char_commons(idx: usize, zero_values: &Vec<i32>, one_values: &Vec<i32>) -> (char, char) {
    if one_values[idx] == 0 {
        ('0', '0')
    } else if zero_values[idx] == 0 {
        ('1', '1')
    } else if one_values[idx] >= zero_values[idx] {
        ('1', '0')
    } else {
        ('0', '1')
    }
}

fn run() -> Result<(), String> {
    let str = std::fs::read_to_string("./src/input.txt").map_err(|e| e.to_string())?;
    let lines: Vec<&str> = str.split("\n").map(|l| l.trim()).collect();
    let width = lines[0].len();
    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    let bit_commons = |idx: usize, zero_values: &Vec<i32>, one_values: &Vec<i32>| -> (u64, u64) {
        if one_values[idx] == 0 {
            (0, 0)
        } else if zero_values[idx] == 0 {
            (1, 1)
        } else if one_values[idx] >= zero_values[idx] {
            (1, 0)
        } else {
            (0, 1)
        }
    };

    let (p1_zero, p1_one) = detect_counts(&lines, &width);
    for idx in 0..width {
        let (most_common_bit, least_common_bit) = bit_commons(idx, &p1_zero, &p1_one);
        gamma = gamma << 1;
        gamma = gamma | most_common_bit;
        epsilon = epsilon << 1;
        epsilon = epsilon | least_common_bit;
    }

    fn find_last_remaining_string_binary_value_by_retention_as_u64(
        lines: &Vec<&str>,
        width: &usize,
        retainment: Box<dyn Fn(char, char, char) -> bool>,
    ) -> u64 {
        let mut remaining = lines.as_slice().to_vec();
        for idx in 0..*width {
            let (zero_values, one_values) = detect_counts(&remaining, width);
            let (most_common_bit, least_common_bit) =
                bit_char_commons(idx, &zero_values, &one_values);
            remaining.retain(|v| {
                let chars: Vec<char> = v.chars().collect();
                retainment(chars[idx], most_common_bit, least_common_bit)
            });
            if remaining.len() == 1 {
                return bin_to_num(remaining[0]);
            }
        }
        panic!("Nothing found")
    }

    let oxy = find_last_remaining_string_binary_value_by_retention_as_u64(
      &lines,
      &width,
      Box::new(
        |cur_bit, most_common_bit, _least_common_bit| cur_bit == most_common_bit
      )
    );
    let co2 = find_last_remaining_string_binary_value_by_retention_as_u64(
      &lines,
      &width,
      Box::new(
        |cur_bit, _most_common_bit, least_common_bit| cur_bit == least_common_bit
      )
    );
    println!("Done! x {} y {} mult {}", oxy, co2, oxy * co2);
    Ok(())
}

fn main() {
    match run() {
        Err(e) => println!("Failed {}", e),
        Ok(_) => (),
    }
}
