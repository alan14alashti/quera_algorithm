use std::io;

pub fn recursive_sequence() {
  let mut n = String::new();
  io::stdin().read_line(&mut n).expect("fail");

  fn calculate(number: i128) -> i128 {
    if number == 0 {
      return 5;
    }

    let temp = calculate(number-1);
    if (number % 2) == 0 {
      return temp - 21;
    } else {
      return temp * temp;
    }
  }

  let result = calculate(n.trim().parse::<i128>().unwrap());
  
  print!("{}", result);
}