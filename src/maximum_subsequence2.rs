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

  for l in 0..first_line.trim().parse::<i128>().unwrap() {
    let mut sum: i128 = 0;
    for r in l..first_line.trim().parse::<i128>().unwrap()  {
      sum = sum + sequence[r as usize].trim().parse::<i128>().unwrap();
      if sum > result as i128 {
        result = sum as f64;
      }
    }
  }

  println!("{}", result as i128);
}
