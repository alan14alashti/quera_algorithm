use std::{io};
use std::iter::FromIterator;

pub fn semiTidy() {
  let mut  count = 0;
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");

  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let sequence = Vec::from_iter(second_line.trim().split(" ").map(String::from));

  for i in 0..first_line.trim().parse::<i128>().unwrap() {
    if i+1 != sequence[(i as usize)].trim().parse::<i128>().unwrap() {
      count += 1
    } 
  }

  if count == 2 {
    println!("YES");
  } else {
    println!("NO");
  }
}