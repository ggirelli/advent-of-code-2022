use crate::utils::falling_sand::scan_to_map;
use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    let _map: Vec<Vec<bool>> = scan_to_map(&_rows);
    0
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day14.test.txt".to_string()), 24);
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
            let sand_units_to_rest: i32 = pt1("data/day14.txt".to_string());
            println!("Total sand units before void: {}", sand_units_to_rest);
            sand_units_to_rest
        }
        2 => pt2("data/day14.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
