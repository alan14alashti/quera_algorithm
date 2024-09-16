use std::io;

pub fn horner() {
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");
  
  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let first_line_vector: Vec<i128> = first_line
  .trim()  
  .split_whitespace() 
  .map(|s| s.parse().expect("Please enter valid numbers")) 
  .collect();

  let n = first_line_vector[0];
  let x = first_line_vector[1];

  let sequence: Vec<i128> = second_line
    .trim()  
    .split_whitespace() 
    .map(|s| s.parse().expect("Please enter valid numbers")) 
    .collect();

  let mode = 1000000007;
  let mut p = 0;
  for i in 0..n+1 {
    p = (p * x) + sequence[i as usize];
    p = p % mode;
    if p < 0 {
      p += mode;
    }
  }

  print!("{}", p);
}