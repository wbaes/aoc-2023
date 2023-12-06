use std::env;
use crate::read_file::read_file::read_file;

pub mod read_file;
pub mod day1;
pub mod day2;
pub mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let day: i32 = args[2].trim().parse().expect("Please type a number as day");
    println!("Parsing: {}", filename);
    let lines = read_file(filename);
    match day {
        1 => day1::day1::calculate(lines),
        2 => day2::day2::calculate(lines),
        6 => day6::day6::calculate(lines),
        _ => println!("please subnit a valid day.")
    };
}
