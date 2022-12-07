use crate::utils::io::read_rows;
use crate::utils::strings::find_stretch_of_unique_characters;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    find_stretch_of_unique_characters(_rows[0].to_string(), 4)
}

pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    find_stretch_of_unique_characters(_rows[0].to_string(), 14)
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let packet_start: i32 = pt1("data/day06.txt".to_string());
            println!("The packet starts at character #{}", packet_start);
            0
        }
        2 => {
            let message_start: i32 = pt2("data/day06.txt".to_string());
            println!("The message starts at character #{}", message_start);
            0
        }
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
