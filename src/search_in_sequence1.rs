use std::{io};

pub fn search_in_sequence1() {
  let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).expect("fail");
    
    let mut iter = first_line.split_whitespace();
    let mut n: i32 = iter.next().unwrap().parse().unwrap();
    let mut q: i32 = iter.next().unwrap().parse().unwrap();

    let mut second_line = String::new();
    io::stdin().read_line(&mut second_line).expect("fail");

    let sequence = second_line.split(" ");
    
    let mut questions = Vec::new();
    for j in 0..q {
        let mut question = String::new();
        io::stdin().read_line(&mut question).expect("fail");
        questions.push(question);
    }

    for question in questions {
        let mut numbers = 0;
        for x in sequence.clone() {
            if (x.trim_end().parse::<i32>().unwrap() < question.trim_end().to_string().parse::<i32>().unwrap()) {
                numbers = numbers + 1;
            }
        }
        println!("{}", numbers)
    }
}