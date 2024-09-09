use std::{cmp::max, io};

pub fn maximum_subsequence3() {
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");
  
  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let sequence: Vec<i128> = second_line
    .trim()  
    .split_whitespace() 
    .map(|s| s.parse().expect("Please enter valid numbers")) 
    .collect();

  let n = first_line.trim().parse::<i128>().unwrap(); 

  let mut result = sequence[0];
  let mut maxsum = sequence[0];
  for i in 1..n {
    maxsum = max(maxsum + sequence[i as usize], sequence[i as usize]);
    result = max(maxsum, result)
  }

  print!("{}", result);
}