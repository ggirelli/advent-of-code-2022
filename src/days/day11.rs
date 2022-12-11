use crate::utils::io::read_rows;
use crate::utils::keep_away;
use crate::utils::keep_away2;

pub fn pt1(file_path: String) -> i64 {
    let _rows: Vec<String> = read_rows(&file_path);
    let mut _monkeys: Vec<keep_away::Monkey> = keep_away::parse_monkeys(&_rows);
    keep_away::run_rounds(&mut _monkeys, 20)
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day11.test.txt".to_string()), 10605);
}

pub fn pt2(file_path: String) -> i64 {
    let _rows: Vec<String> = read_rows(&file_path);
    let modulus_base = keep_away2::update_modulus_base(&_rows, 1);
    let mut _monkeys: Vec<keep_away2::Monkey> = keep_away2::parse_monkeys(&_rows, &modulus_base);
    keep_away2::run_rounds(&mut _monkeys, 10000)
}

#[test]
fn test_pt2() {
    assert_eq!(pt2("data/day11.test.txt".to_string()), 2713310158);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let monkey_business_level: i64 = pt1("data/day11.txt".to_string());
            println!("Total monkey business level: {}", monkey_business_level);
            0
        }
        2 => {
            let monkey_business_level: i64 = pt2("data/day11.txt".to_string());
            println!("Total monkey business level: {}", monkey_business_level);
            0
        }
        _ => panic!("Part {} not found.", part),
    };
}
