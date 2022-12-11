use crate::utils::io::read_rows;
use crate::utils::keep_away::parse_monkeys;
use crate::utils::keep_away::Monkey;
use crate::utils::modulus::Modulus;

fn run_rounds(_monkeys: &mut Vec<Monkey>, n_rounds: usize, manage_worry: bool) -> i64 {
    for round_idx in 0..n_rounds {
        println!("{}", round_idx);
        for monkey_idx in 0.._monkeys.len() {
            while _monkeys[monkey_idx].items.len() > 0 {
                let mut inspection_ans: (Modulus, usize) =
                    _monkeys[monkey_idx].inspect_next(manage_worry);

                inspection_ans
                    .0
                    .change_modulus(_monkeys[inspection_ans.1].test);

                _monkeys[inspection_ans.1].items.push(inspection_ans.0);
            }
        }
    }

    let mut inspection_counters: Vec<i64> = Vec::new();
    for monkey in _monkeys {
        inspection_counters.push(monkey.inspection_counter as i64);
    }
    inspection_counters.sort();
    println!("{:?}", inspection_counters);
    inspection_counters[inspection_counters.len() - 1]
        * inspection_counters[inspection_counters.len() - 2]
}

pub fn pt1(file_path: String) -> i64 {
    let _rows: Vec<String> = read_rows(&file_path);
    let mut _monkeys: Vec<Monkey> = parse_monkeys(&_rows);
    run_rounds(&mut _monkeys, 20, true)
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day11.test.txt".to_string()), 10605);
}

pub fn pt2(file_path: String) -> i64 {
    let _rows: Vec<String> = read_rows(&file_path);
    let mut _monkeys: Vec<Monkey> = parse_monkeys(&_rows);
    run_rounds(&mut _monkeys, 10000, false)
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
