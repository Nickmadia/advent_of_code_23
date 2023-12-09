use std::collections::HashSet;

use crate::file_reader;


pub fn solve() {
    let v: Vec<String> = file_reader::get_file_v_str("input/7.txt");

    let mut vals : Vec<Vec<&str>>= v.iter()
                        .map(|x| x.split(' ').collect())
                        .collect();
    
    vals.sort_by(| a, b | compare_hands(a[0], b[0]));
    
    let mut res = 0; 
    for i in 0..vals.len()  {
        res += (i as i32+1) * vals[i][1].parse::<i32>().expect("error parsing");
    } 
    println!("Result: {}", res);
}

fn card_value(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

fn hand_value(hand: &str) -> Vec<u8> {
    hand.chars().map(|c| card_value(c)).collect()
}
fn get_hand_type(hand: &str) -> i32{
    let chars: Vec<char> = hand.chars().collect();
    let mut set = HashSet::new();
    for x in chars.iter() {
        set.insert(*x);
    }
    
    match set.len() {
        1 => 7,
        2 => { if set.iter().any(|x| chars.iter().filter(|y| *y == x).count() == 4) {
                6    
            } else {
                5
            }
        }
        3 => { if set.iter().any(|x| chars.iter().filter(|y| *y == x).count() == 3) {
                4
            } else {
                3
            }
        }
        4 => 2,
        5 => 1,
        _ => 0
    }
}
fn compare_hands(hand1: &str, hand2: &str) -> std::cmp::Ordering {
    let h1 = get_hand_type(hand1);
    let h2 = get_hand_type(hand2);
    //println!("{}:{}  {}:{}",h1,hand1,h2,hand2);
    if h1 != h2 {
       h1.cmp(&h2) 
    } else {
    let value1 = hand_value(hand1);
    let value2 = hand_value(hand2);
    for i in 0..5 {
        match value1[i].cmp(&value2[i]) {
            std::cmp::Ordering::Equal => continue,
            result => return result,
        }
    }

    std::cmp::Ordering::Equal
    }
}
