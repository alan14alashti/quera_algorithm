use std::io;

pub fn counting_triangles() {
  let mut result: i32 = 0;
  let mut n = String::new();
  io::stdin().read_line(&mut n).expect("fail");

  for a in 1..n.trim().parse::<i32>().unwrap() + 1 {
    for b in a..n.trim().parse::<i32>().unwrap() + 1  {
      for c in b..n.trim().parse::<i32>().unwrap() + 1 {
        if ((a + b + c) == n.trim().parse::<i32>().unwrap()) & ((a + b) > c) {
          result = result + 1;
        }
      }
    }
  }

  println!("{}", result);
}