use std::env;

mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please run specifying a day and a part.");
        println!("e.g., 'cargo run 1 1' for day 01 part 1")
    } else {
        match args[1].parse::<i32>().unwrap() {
            1 => days::day01::run(args[2].parse::<i32>().unwrap()),
            2 => days::day02::run(args[2].parse::<i32>().unwrap()),
            3 => days::day03::run(args[2].parse::<i32>().unwrap()),
            4 => days::day04::run(args[2].parse::<i32>().unwrap()),
            5 => days::day05::run(args[2].parse::<i32>().unwrap()),
            6 => days::day06::run(args[2].parse::<i32>().unwrap()),
            7 => days::day07::run(args[2].parse::<i32>().unwrap()),
            8 => days::day08::run(args[2].parse::<i32>().unwrap()),
            _ => panic!("Day {} not found.", args[1]),
        }
    }
}
