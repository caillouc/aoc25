use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("You must provide one argument (the day number");
    }

    let day = &args[1];
    let file_path = format!("data/{day}.txt");
    let data =
        fs::read_to_string(&file_path).expect(format!("Cannot read file {}", file_path).as_str());

    match day.as_str() {
        "day1" => day1::solve(data),
        "day2" => day2::solve(data),
        "day3" => day3::solve(data),
        "day4" => day4::solve(data),
        "day5" => day5::solve(data),
        "day6" => day6::solve(data),
        "day7" => day7::solve(data),
        day => println!("Invalid day {}", day),
    }
}
