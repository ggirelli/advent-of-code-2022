use crate::utils::io::read_rows;
use crate::utils::strings::find_stretch_of_unique_characters;

fn find_packet_start(content: String) -> i32 {
    find_stretch_of_unique_characters(content, 4)
}

fn find_message_start(content: String) -> i32 {
    find_stretch_of_unique_characters(content, 14)
}

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    find_packet_start(_rows[0].to_string())
}

pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    find_message_start(_rows[0].to_string())
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
