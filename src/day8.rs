use std::collections::HashMap;
use crate::file_reader;

pub fn solve() {
    let mut dictionary: HashMap<String, (String, String)> = HashMap::new();

    let input_lines = file_reader::get_file_v_str("input/8.txt");
    
    let directions:Vec<char> = input_lines[0].chars().collect();
    for line in &input_lines[1..] {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();

        if parts.len() == 2 {
            let key = parts[0].trim().to_string();

            let value_parts: Vec<&str> = parts[1]
                .trim_matches(|p| p == '(' || p == ')' || p == ' ')
                .split(',')
                .map(|s| s.trim())
                .collect();

            if value_parts.len() == 2 {
                let tuple_value = (
                    value_parts[0].to_string(),
                    value_parts[1].to_string(),
                );

                dictionary.insert(key, tuple_value);
            }
        }
    }
    let mut target ="AAA";
    let mut res =0;
    while target != "ZZZ" {
        for c in &directions {
            res += 1 ;
            match c {
                'L' => {
                    target = &dictionary.get_key_value(target).unwrap().0;
                },
                'R' => {
                    target = &dictionary.get_key_value(target).unwrap().1.1;
                },
                _ => continue
           } 
           
        }
    } 
    println!("Result: {}", res);

}
