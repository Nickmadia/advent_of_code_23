
use std::io::BufRead;

use crate::file_reader;



pub fn solve() {
    let input = file_reader::read_file("input/5.txt");
    let lines: Vec<String> = input.lines()
                        .map(|x| x.unwrap())
                        .map(String::from)
                        .collect();
    let seeds: Vec<i64> = lines[0].split(':')
                        .last()
                        .unwrap()
                        .split(' ')
                        .map(str::parse::<i64>)
                        .filter(|x| x.is_ok())
                        .map(|x| x.expect("tha"))
                        .collect();
    let mut map: Vec<Vec<(i64,i64,i64)>> = vec![];
    let mut i = 1;
    while i < lines.len() {
        if let Some(first_char) = lines[i].chars().next() {
            if first_char.is_alphabetic() {
                i += 1;
                let mut temp: Vec<(i64, i64, i64)> = vec![];
                while i < lines.len() && !lines[i].is_empty() {
                    let t: Vec<i64> = lines[i]
                        .split(" ")
                        .map(|x| x.parse::<i64>())
                        .map(|x| x.expect("Errore durante il parsing"))
                        .collect();
                    temp.push((t[0], t[1], t[2]));
                    i += 1;
                }
                map.push(temp);
            }
        } else {
            // Se lines[i] è una stringa vuota o non contiene caratteri
            i += 1;
        }
    }
    let mut min :i64= -1;
    for seed in seeds {
        let mut cur = seed;
        for istr in &map {
            for x in istr {
                if cur >= x.1 && cur <= x.1 + x.2{
                    cur = cur - (x.1 -x.0);
                    
                    break
                }
            }
        }
        if min == -1 {min = cur}
        if cur < min {min = cur;}
    }

    println!("Result: {}", min);

}