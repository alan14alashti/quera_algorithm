use std::io;

pub fn selection_sort() {
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
    let mut min_index = i;
    for j in i..n {
      if sequence[min_index as usize] > sequence[j as usize] {
        min_index = j;
      }
    }

    sequence.swap(i as usize, min_index as usize)
  }

  for k in sequence {
    print!("{} ", k);
  }
}