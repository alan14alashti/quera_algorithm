use std::io;

pub fn counting_triangles3() {
  let mut result: i128 = 0;
  let mut first_line = String::new();
  io::stdin().read_line(&mut first_line).expect("fail");
  let n = first_line.trim().parse::<i128>().unwrap();
  for a in 0..(((n)/3) as f32).floor() as i128 + 1{
    let upper = 0.max(((n/2) as f32).floor() as i128 - (2*a) +1);
    let lower = (((n-(3*a))/2) as f32).floor() as i128;
    result = result + lower - upper + 1;
  }

  println!("{}", result);
}