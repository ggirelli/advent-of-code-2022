use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    0
}

#[test]
fn test_pt1() {}

pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    0
}

#[test]
fn test_pt2() {}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => pt1("data/day12.txt".to_string()),
        2 => pt2("data/day12.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
