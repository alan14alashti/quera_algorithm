use std::io;
use std::iter::FromIterator;

pub fn arithmetic_squence2() {
  let mut result: i64 = 0;
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");
  
  let mut iter = first_line.split_whitespace();
  let n: i64 = iter.next().unwrap().parse().unwrap();
  let k: i64 = iter.next().unwrap().parse().unwrap();

  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let sequence = Vec::from_iter(second_line.split(" ").map(String::from));
  
  let mut b = vec!(0; n as usize);
  for i in 1..n+1 {
    b[i as usize - 1] = sequence[i as usize - 1].trim().parse::<i64>().unwrap() - (i - 1) * k
  }

  b.sort();
  let median: i64;
  if n % 2 == 0 {
    median = (b[((n / 2) - 1) as usize] + b[((n + 1) / 2 - 1) as usize]) / 2;
  } else {
    median = b[((n as usize)) / 2];
  }

  for i in 0..n {
    result = result + (b[i as usize] - median).abs();
  }
  println!("{}", result);
}