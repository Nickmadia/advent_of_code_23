use std::io::BufRead;

use crate::file_reader;



pub fn solve() {
    let mut res: i32 = 0;
    let input = file_reader::read_file("input/4.txt");
    for line in input.lines() {
        let temp = line.unwrap();
        let v: Vec<Vec<&str>> = temp.split(':')
                                .last()
                                .unwrap()
                                .split('|')
                                .map(|x| x.split(' ').collect()).collect() ;

        let wins = v[0].iter()
                        .filter(|x| (!x.is_empty() && v[1].contains(x)))
                        .count(); 
        if wins >0 {
            res += 2_i32.pow((wins -1 ) as u32);
        }
    };
    println!("Result: {}", res);
}