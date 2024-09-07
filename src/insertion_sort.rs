use std::io;

pub fn insertion_sort() {
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");
  
  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let mut sequence: Vec<i32> = second_line
    .trim()  
    .split_whitespace() 
    .map(|s| s.parse().expect("Please enter valid numbers")) 
    .collect();

  let n = first_line.trim().parse::<i128>().unwrap(); 

  for i in 0..n {
    let mut p = i;
    while p > 0 && sequence[p as usize] < sequence[p as usize - 1] {
      sequence.swap(p as usize, p as usize - 1);
      p = p -1;
    }
  }

  for k in sequence {
    print!("{} ", k);
  }
}