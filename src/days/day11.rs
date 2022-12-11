use crate::utils::io::read_rows;
use crate::utils::keep_away::parse_monkeys;
use crate::utils::keep_away::Monkey;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);

    let mut _monkeys: Vec<Monkey> = parse_monkeys(&_rows);
    for _ in 0..20 {
        for monkey_idx in 0.._monkeys.len() {
            while _monkeys[monkey_idx].items.len() > 0 {
                let inspection_ans: (i32, usize) = _monkeys[monkey_idx].inspect_next();
                _monkeys[inspection_ans.1].items.push(inspection_ans.0);
            }
        }
    }

    let mut inspection_counters: Vec<i32> = Vec::new();
    for monkey in _monkeys {
        inspection_counters.push(monkey.inspection_counter as i32);
    }
    inspection_counters.sort();
    inspection_counters[inspection_counters.len() - 1]
        * inspection_counters[inspection_counters.len() - 2]
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day11.test.txt".to_string()), 10605);
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
            let monkey_business_level: i32 = pt1("data/day11.txt".to_string());
            println!("Total monkey business level: {}", monkey_business_level);
            monkey_business_level
        }
        2 => pt2("data/day11.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
