use std::env;

mod day01;
mod day02;
mod day03;
mod io;
mod strings;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please run specifying a day and a part.");
        println!("e.g., 'cargo run 1 1' for day 01 part 1")
    } else {
        match args[1].parse::<i32>().unwrap() {
            1 => day01::run(args[2].parse::<i32>().unwrap()),
            2 => day02::run(args[2].parse::<i32>().unwrap()),
            3 => day03::run(args[2].parse::<i32>().unwrap()),
            _ => println!("Day {} not found.", args[1]),
        }
    }
}
