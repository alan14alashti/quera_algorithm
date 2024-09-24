use std::io;

pub fn ordered_n_tuples() {
  let mut n: String = String::new();
  io::stdin().read_line(&mut n).expect("fail");

  let length = n.trim().parse::<usize>().unwrap();

  let mut current_sequence = Vec::new();
  let mut result = Vec::new();
  
  fn generate_recursive(len: usize, current_sequence: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if current_sequence.len() == len {
      result.push(current_sequence.clone());
      return;
    }

    for i in 1..=len as i32 {
      current_sequence.push(i);
      generate_recursive(len, current_sequence, result); // Recur with the next element
      current_sequence.pop(); // Backtrack to generate other sequences
    }
  }

  generate_recursive(length, &mut current_sequence, &mut result);

  // Print the results
  for seq in result {
    for num in seq {
      print!("{} ", num);
    }
    println!();
  }
}