use std::fs;
struct Data {
    number: u32,
    valid: bool,
}
pub fn solve() {
    // Read the input file
    let input = fs::read_to_string("input/3.txt")
        .expect("Could not read file");

    // Parse the input into a 2D vector of characters
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        schematic.push(row);
    }

    // Calculate the sum of adjacent part numbers
    let mut sum = 0;
    let rows = schematic.len();
    let cols = schematic[0].len();
    let mut num = Data {
        number: 0,
        valid: false,
    }; 
    for i in 0..rows {
        for j in 0..cols {
            if schematic[i][j].is_digit(10) {
                // Check all adjacent positions
                num.number = num.number*10 + schematic[i][j].to_digit(10).unwrap() ;
                for &di in &[-1, 0, 1] {
                    for &dj in &[-1, 0, 1] {
                        let new_i = i as i32 + di;
                        let new_j = j as i32 + dj;

                        // Check if new position is within bounds and not a period (.)
                        if new_i >= 0 && new_i < rows as i32 &&
                           new_j >= 0 && new_j < cols as i32 &&
                           is_symbol(schematic[new_i as usize][new_j as usize])  {
                           num.valid = true;
                            //sum += schematic[i][j].to_digit(10).unwrap() as usize;
                        }
                    }
                }
            }else if num.number >0 {
                
                if num.valid {
                    sum += num.number;
                    println!("{} : {}",num.number, sum);
                } 
                num.number = 0;
                num.valid = false;
            }
        }
    }

    println!("The sum of all part numbers in the engine schematic is: {}", sum);
}
fn is_symbol(c: char) -> bool {
   !c.is_digit(10) && c != '.' || c =='*'
}
