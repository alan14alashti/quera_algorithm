use std::io;

pub fn bubble_sort() {
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");
  
  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let mut sequence: Vec<i32> = second_line
    .trim()  
    .split_whitespace() 
    .map(|s| s.parse().expect("Please enter valid numbers")) 
    .collect();

  let n = first_line.trim().parse::<i128>().unwrap() - 1;

  for i in 0..n {
    for j in 0..n-i {
      if sequence[j as usize] > sequence[(j as usize) + 1] {
        sequence.swap(j as usize + 1, j as usize)
      }
    }
  }

  for k in sequence {
    print!("{} ", k)
  }
}