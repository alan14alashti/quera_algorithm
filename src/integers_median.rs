use std::io;
use std::iter::FromIterator;

pub fn integers_median() {
  let median: i32;
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");

  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let mut sequence = Vec::from_iter(second_line.trim().split(" ").map(String::from));

  let mut b = vec!(0; first_line.trim().parse::<i64>().unwrap() as usize);
  for i in 0..first_line.trim().parse::<i64>().unwrap() {
    b[i as usize] = sequence[i as usize].trim().parse::<i32>().unwrap();
  }

  b.sort();

  if first_line.trim().parse::<i64>().unwrap() % 2 == 1 {
    median = b[(((first_line.trim().parse::<i64>().unwrap() + 1) / 2) - 1) as usize];
  } else {
    median = b[(((first_line.trim().parse::<i64>().unwrap()) / 2) - 1) as usize];
  }

  let mut result: i64 = 0;

  for i in 0..first_line.trim().parse::<i64>().unwrap() {
    result = result + (b[i as usize] - median).abs() as i64;
  }

  println!("{} {}", median, result);
}