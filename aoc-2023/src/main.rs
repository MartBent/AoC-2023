mod day1;
mod day2;
mod day3;

use std::fs;

const CURRENT_DAY: i32 = 3;

fn main() {
    let filepath = format!("src/day{}/input.txt", CURRENT_DAY);
    let contents = fs::read_to_string(filepath).expect("Failed to read file");

    match CURRENT_DAY {
        1 => day1::run(contents),
        2 => day2::run(contents),
        3 => day3::run(contents),
        _ => println!("Day {} not implemented", CURRENT_DAY),
    }
}