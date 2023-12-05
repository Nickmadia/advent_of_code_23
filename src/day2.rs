use std::io::BufRead;

use crate::file_reader;


const R: i32= 12;
const G: i32= 13;
const B: i32= 14;

pub fn solve() {
    let input = file_reader::read_file("input/2.txt");
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut res = 0;
    let mut valid = true;
    for line in input.lines() {
        let temp = line.unwrap();
        let v: Vec<&str> = temp.split(':').collect();
        let id = v[0].split(' ').last().unwrap().parse::<i32>().unwrap();
        let sets: Vec<&str> = v[1].split(';').collect();

        for set in sets {
            let ext: Vec<&str> = set.split_whitespace().collect();
            let c_ext:Vec<&str> = ext.clone();
            let mut index = 0;
            for val in ext {
                if index % 2 == 0 {
                    if c_ext[index+1].contains("red") {
                        r += val.parse::<i32>().unwrap();
                    } else if c_ext[index+1].contains("blue") {
                        b += val.parse::<i32>().unwrap();
                    } else { 
                        g += val.parse::<i32>().unwrap();
                    }
                }
                index +=1;
            }
            if r > R || g > G || b > B {
                valid = false;
                r = 0;
                g = 0;
                b = 0;

                break  ;              
            }
           
            r = 0;
            g = 0;
            b = 0;

            }
            if valid {
                res +=id;
            }
            valid = true;
        }
        println!("Result: {}", res);

    }

