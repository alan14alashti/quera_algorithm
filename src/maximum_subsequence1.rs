use std::f64::INFINITY;
use std::io;
use std::iter::FromIterator;

pub fn maximum_subsequence1() {
  let mut result = -INFINITY;
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");

  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let sequence = Vec::from_iter(second_line.trim().split(" ").map(String::from));

  for r in 0..first_line.trim().parse::<i128>().unwrap() {
    for l in 0..r+1  {
      let mut sum: i128 = 0;
      for i in l..r+1 {
        sum = sum + sequence[i as usize].trim().parse::<i128>().unwrap()
      }
      if sum > result as i128 {
        result = sum as f64;
      }
    }
  }

  println!("{}", result as i128);
}
