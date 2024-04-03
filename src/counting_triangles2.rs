use std::io;

pub fn counting_triangles2() {
  let mut result: i32 = 0;
  let mut n = String::new();
  io::stdin().read_line(&mut n).expect("fail");

  for a in 1..n.trim().parse::<i32>().unwrap() + 1 {
    for b in a..n.trim().parse::<i32>().unwrap() + 1  {
      let c =  n.trim().parse::<i32>().unwrap() - (a + b);
      if ((a + b) > c) & (c >= b) {
        result = result + 1;
      }
    }
  }

  println!("{}", result);
}