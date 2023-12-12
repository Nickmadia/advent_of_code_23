mod day1;
mod file_reader;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const MAX_DAYS : usize = 9;
fn main() {
    let solutions: [fn(); MAX_DAYS] = [
        day1::solve1,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
        day8::solve,
        day9::solve
    ];

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <day_number (1-{MAX_DAYS}, or 0 for all days)>", args[0]);
        return;
    }

    let day_to_solve: usize= match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number provided.");
            return;
        }
    };

    if day_to_solve == 0 {
        println!("Running solutions for all days:");
        for (index, _) in solutions.iter().enumerate() {
            solve_day(solutions, index);
        }
    } else if day_to_solve >= 1 && day_to_solve <= MAX_DAYS {
        solve_day(solutions, day_to_solve - 1);
    } else {
        println!("Invalid day number provided. Day number should be between 1 and 25, or 0 for all days.");
    }
}

fn solve_day(solutions: [fn(); MAX_DAYS], day_number: usize) {
    match solutions.get(day_number) {
        Some(solution) => {
            println!("Day {} solution:", day_number + 1);
            solution();
        }
        None => println!("Day {} not implemented yet.", day_number + 1),
    }
}

