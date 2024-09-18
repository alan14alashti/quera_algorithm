use std::io;

pub fn sum_of_digits() {
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");

  let first_line_vector: Vec<u32> = first_line
  .trim()
  .chars()
  .filter_map(|c| c.to_digit(10)) // Convert each character to a digit
  .collect();

  let n = first_line_vector.len();

  let mut sum = 0;
  for i in 0..n {
    sum += first_line_vector[i];
  }

  print!("{}", sum);
}