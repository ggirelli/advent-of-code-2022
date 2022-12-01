use std::env;
mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please run specifying a day and a part.");
        println!("e.g., 'cargo run 1 1' for day 01 part 1")
    } else {
        if args[1] == "1" {
            if args[2] == "1" {
                day01::pt1();
            }
            if args[2] == "2" {
                day01::pt2();
            }
        }
    }
}