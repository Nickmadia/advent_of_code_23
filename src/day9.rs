use std::io::BufRead;

use crate::file_reader;

pub fn solve(){
    let values = file_reader::read_file("input/9.txt");
    let mut res = 0;
    for line in values.lines() {

        let vals: Vec<i32> = line.expect("error reading")
                            .split(' ')
                            .map(|x| x.parse::<i32>().expect("error parsing"))
                            .collect();
        res += get_prediction(&vals);
    }
    println!("Result: {}",res);
}

fn get_prediction(values: &Vec<i32>) -> i32 {
    let mut next: Vec<i32> = Vec::new();
    let mut n: i32 = 0;
    for i in 0..values.len() -1{
        next.push(values[i+1]-values[i]);
    }
    if !next.iter().all(|x| *x==0) {
        n = get_prediction(&next);
    }
    n + values[values.len()-1]
}