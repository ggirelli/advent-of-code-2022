use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    0
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day15.test.txt".to_string()), 26);
}

pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    0
}

#[test]
fn test_pt2() {}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let positions_count: i32 = pt1("data/day15.txt".to_string());
            println!("{} positions", positions_count);
            positions_count
        }
        2 => pt2("data/day15.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
