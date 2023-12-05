use std::io::BufRead;

use crate::file_reader;

pub fn solve1() {
    let mut res = 0;
    let mut first:i32 = -1;
    let mut last:i32 = -1;
    let input = file_reader::read_file("input/1.txt");
    for line in input.lines() {
       match line {
        Ok(text) => {
            
            for c in text.chars() {
                if c.is_numeric() {
                    if first == -1 {
                        first = c.to_digit(10).unwrap() as i32;
                        last = first;
                    } else {
                        last = c.to_digit(10).unwrap() as i32;
                    }
                } 
            }
            res += first *10 + last;
            first = -1;
            last = first;
        } 
        Err(err) => println!("{}",err),
       } 
    }
    println!("Result: {}", res);
    
}


