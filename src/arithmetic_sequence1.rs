use std::f32::INFINITY;
use std::{io};
use std::iter::FromIterator;

pub fn arithmetic_squence1() {
  let mut result: f32 = INFINITY;
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");
  
  let mut iter = first_line.split_whitespace();
  let mut n: i32 = iter.next().unwrap().parse().unwrap();
  let mut k: i32 = iter.next().unwrap().parse().unwrap();

  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let mut sequence = Vec::from_iter(second_line.split(" ").map(String::from));
  
  let mut maximum = sequence[0].trim().parse::<i32>().unwrap();
  let mut minimum = sequence[0].trim().parse::<i32>().unwrap();

  for x in sequence.clone() {
    if x.trim().parse::<i32>().unwrap() > maximum {
      maximum = x.trim().parse::<i32>().unwrap();
    } 

    if x.trim().parse::<i32>().unwrap() < minimum {
      minimum = x.trim().parse::<i32>().unwrap();
    } 
  }

  for x in (minimum - (n - 1)*k)..maximum+1 {
    let mut cost = 0;
    for i in 1..n+1 {
      let z = x + ((i - 1)*k) - sequence[(i - 1) as usize].trim().parse::<i32>().unwrap();
      // println!("{}", z);
      cost = cost + z.abs();
    }
    if cost < result as i32 {
      result = cost as f32;
    }
  }

  println!("{}", result);
}