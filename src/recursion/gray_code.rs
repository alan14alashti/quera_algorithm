use std::io;

pub fn gray_code() {
  let mut n: String = String::new();
  io::stdin().read_line(&mut n).expect("fail");

  let length = n.trim().parse::<usize>().unwrap();

  fn generate_recursive(len: usize) -> Vec<String> {
    if len == 1 {
      return vec!["0".to_string(), "1".to_string()];
    } 

    let g_code = generate_recursive(len - 1);

    let mut result = Vec::new();

    for i in &g_code {
      result.push(format!("0{}", i));
    }

    for j in g_code.iter().rev() {
      result.push(format!("1{}", j));
    }

    result
  }

  let gray_code = generate_recursive(length);

  for code in gray_code {
    println!("{}", code);
  }
}