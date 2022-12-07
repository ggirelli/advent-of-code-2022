use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    0
}

#[test]
fn test_pt1() {}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => pt1("data/day06.txt".to_string()),
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
