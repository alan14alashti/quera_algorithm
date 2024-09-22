use std::io;

pub fn honoi() {
  let mut n = String::new();
  io::stdin().read_line(&mut n).expect("fail");

  let mut step: u32= 1;
  
  fn print_honoi(number:u32, src: char, dst: char, help: char, st: &mut u32) {
    if number == 1 {
      println!("{} {} {}",*st,src,dst);
      *st += 1;
      return 
    }
    print_honoi(number-1,src,help,dst, st);
    println!("{} {} {}",*st,src,dst);
    *st += 1;
    print_honoi(number-1,help,dst,src, st)

  }

  print_honoi(n.trim().parse::<u32>().unwrap(),'A','B','C', &mut step);
}