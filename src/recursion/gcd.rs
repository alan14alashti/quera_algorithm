use std::io;

pub fn gcd() {
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");

  let first_line_vector: Vec<u128> = first_line
  .trim()  
  .split_whitespace() 
  .map(|s| s.parse().expect("Please enter valid numbers")) 
  .collect();

  let a: u128;
  let b: u128;


  if first_line_vector[0] < first_line_vector[1] {
    b = first_line_vector[0];
    a = first_line_vector[1];
  } else {
    a = first_line_vector[0];
    b = first_line_vector[1];
  }

  fn findgcd(x: u128, y: u128) -> u128 {
    if y == 0 {
      return x;
    } else {
      return findgcd(y, x % y)
    }
  }

  let result = findgcd(a, b);
  print!("{}", result);
}