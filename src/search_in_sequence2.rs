use std::{io};
use std::iter::FromIterator;
pub fn search_in_sequence2() {
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");
  
  let mut iter = first_line.split_whitespace();
  let mut n: i32 = iter.next().unwrap().parse().unwrap();
  let mut q: i32 = iter.next().unwrap().parse().unwrap();

  let mut second_line = String::new();
  io::stdin().read_line(&mut second_line).expect("fail");

  let mut sequence = Vec::from_iter(second_line.split(" ").map(String::from));
  
  let mut questions = Vec::new();
  for j in 0..q {
      let mut question = String::new();
      io::stdin().read_line(&mut question).expect("fail");
      questions.push(question);
  }
  
  let mut sequence_maximum = sequence[1].trim_end().to_string().parse::<i32>().unwrap();
  for i in 2..n {
    if sequence[i as usize].trim_end().to_string().parse::<i32>().unwrap()>sequence_maximum {
      sequence_maximum = sequence[i as usize].trim_end().to_string().parse::<i32>().unwrap();
    }
  }
  let mut counts = vec![0; sequence_maximum as usize +1];
  for x in 0..n {
    counts[sequence[x as usize].trim_end().to_string().parse::<i32>().unwrap() as usize] = counts[sequence[x as usize].trim_end().to_string().parse::<i32>().unwrap() as usize] + 1;
  }
  
  let mut ps = vec![0; sequence_maximum as usize];

  for i in 1..sequence_maximum {
    // println!("{}", counts[i as usize]);
    ps[i as usize] = ps[i as usize - 1] + counts[i as usize]
  }


  for question in questions {
    if question.trim_end().parse::<i32>().unwrap() > sequence_maximum {
      println!("{}", n);
    } else {
      println!("{}", ps[(question.trim_end().parse::<i32>().unwrap() -1) as usize])
    }
  }
  // for x in counts {
  //   println!("{}", x);
  // }
}