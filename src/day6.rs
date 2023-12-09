
use crate::file_reader;


pub fn solve() {
    let v: Vec<String> = file_reader::get_file_v_str("input/6.txt");

    let vals: Vec<Vec<&str>> = v.iter()
                            .map(|a| a.split(':')
                                    .last()
                                    .unwrap()
                                    .split(' ')
                                    .filter(|s| !s.is_empty())
                                    .collect())
                            .collect();

    let mut m_wins = 1;

    for i in 0..vals[0].len() {
        let time = vals[0][i].parse::<i32>().expect("parse error time");
        let dist = vals[1][i].parse::<i32>().expect("parse error dist");
        let w =  get_w_number(time, dist);
        if w > 0 {
            m_wins = m_wins * w;
        }
    } 

    println!("Result: {}", m_wins);
}

fn get_w_number(time : i32, dist : i32) -> i32{
    let mut wins = 0;
    for i in 0..time {
        if i * (time-i) > dist {
            wins += 1;
        }
    }
    wins
}