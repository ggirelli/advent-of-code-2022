fn _parse_starting_items(line: &str) -> Vec<i32> {
    assert_eq!(&line[..18], "  Starting items: ");
    line[18..]
        .split(", ")
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[test]
fn test_parse_starting_items() {
    assert_eq!(
        _parse_starting_items("  Starting items: 79, 98"),
        [79, 98].to_vec()
    );
    assert_eq!(
        _parse_starting_items("  Starting items: 54, 65, 75, 74"),
        [54, 65, 75, 74].to_vec()
    );
    assert_eq!(
        _parse_starting_items("  Starting items: 79, 60, 97"),
        [79, 60, 97].to_vec()
    );
    assert_eq!(
        _parse_starting_items("  Starting items: 74"),
        [74].to_vec()
    );
}

#[test]
#[should_panic]
fn test_parse_starting_items_panic() {
    _parse_starting_items("Starting items: 79, 98");
}

fn _extract_operation(line: &str) -> String {
    assert_eq!(&line[..13], "  Operation: ");
    line[13..].to_string()
}

#[test]
fn test_extract_operation() {
    assert_eq!(
        _extract_operation("  Operation: new = old * 19"),
        "new = old * 19".to_string()
    );
    assert_eq!(
        _extract_operation("  Operation: new = old + 6"),
        "new = old + 6".to_string()
    );
    assert_eq!(
        _extract_operation("  Operation: new = old * old"),
        "new = old * old".to_string()
    );
    assert_eq!(
        _extract_operation("  Operation: new = old + 3"),
        "new = old + 3".to_string()
    );
}

#[test]
#[should_panic]
fn test_extract_operation_panic() {
    _extract_operation("Operation: new = old * 19");
}

fn _parse_test(line: &str) -> i32 {
    assert_eq!(&line[..21], "  Test: divisible by ");
    line[21..].parse::<i32>().unwrap()
}

#[test]
fn test_parse_test() {
    assert_eq!(_parse_test("  Test: divisible by 23"), 23);
    assert_eq!(_parse_test("  Test: divisible by 19"), 19);
    assert_eq!(_parse_test("  Test: divisible by 13"), 13);
    assert_eq!(_parse_test("  Test: divisible by 17"), 17);
}

#[test]
#[should_panic]
fn test_parse_test_panic() {
    _parse_test("Test: divisible by ");
}

fn _parse_target_true(line: &str) -> usize {
    assert_eq!(&line[..29], "    If true: throw to monkey ");
    line[29..].parse::<usize>().unwrap()
}

fn _parse_target_false(line: &str) -> usize {
    assert_eq!(&line[..30], "    If false: throw to monkey ");
    line[30..].parse::<usize>().unwrap()
}

#[test]
fn test_parse_target() {
    assert_eq!(
        _parse_target_true("    If true: throw to monkey 2"),
        2
    );
    assert_eq!(
        _parse_target_true("    If true: throw to monkey 1"),
        1
    );
    assert_eq!(
        _parse_target_true("    If true: throw to monkey 0"),
        0
    );

    assert_eq!(
        _parse_target_false("    If false: throw to monkey 3"),
        3
    );
    assert_eq!(
        _parse_target_false("    If false: throw to monkey 0"),
        0
    );
    assert_eq!(
        _parse_target_false("    If false: throw to monkey 1"),
        1
    );
}

#[test]
#[should_panic]
fn test_parse_target_true_panic() {
    _parse_target_true("   If true: throw to monkey 0");
}

#[test]
#[should_panic]
fn test_parse_target_false_panic() {
    _parse_target_false("   If true: throw to monkey 0");
}

fn _perform_operation(operation_string: &String, n: i32) -> i32 {
    assert!(operation_string.len() >= 13);

    let value: i32 = match &operation_string[12..] {
        "old" => n,
        _ => operation_string[12..].parse::<i32>().unwrap(),
    };

    let operator_char: char = operation_string
        .chars()
        .nth(10)
        .expect("ERROR: operation string is too short.");
    match operator_char {
        '*' => n * value,
        '+' => n + value,
        _ => panic!(
            "ERROR: unrecognized '{}' operation. ({})",
            operator_char, operation_string
        ),
    }
}

#[test]
fn test_perform_operation() {
    assert_eq!(_perform_operation(&"new = old * 19".to_string(), 1), 19);
    assert_eq!(_perform_operation(&"new = old * 19".to_string(), 3), 57);
    assert_eq!(_perform_operation(&"new = old + 6".to_string(), 1), 7);
    assert_eq!(_perform_operation(&"new = old + 6".to_string(), 57), 63);
    assert_eq!(_perform_operation(&"new = old * old".to_string(), 1), 1);
    assert_eq!(_perform_operation(&"new = old * old".to_string(), 5), 25);
}

#[test]
#[should_panic]
fn test_perform_operation_panic() {
    _perform_operation(&"new = old * ".to_string(), 1);
}

pub struct Monkey {
    pub items: Vec<i32>,
    operation: String,
    test: i32,
    target_true: usize,
    target_false: usize,
    pub inspection_counter: usize,
}

impl Monkey {
    pub fn new(_rows: &Vec<&String>) -> Monkey {
        assert_eq!(_rows.len(), 6);
        Monkey {
            items: _parse_starting_items(_rows[1]),
            operation: _extract_operation(_rows[2]),
            test: _parse_test(_rows[3]),
            target_true: _parse_target_true(_rows[4]),
            target_false: _parse_target_false(_rows[5]),
            inspection_counter: 0,
        }
    }

    pub fn inspect_next(&mut self) -> (i32, usize) {
        assert!(!self.items.is_empty());

        let mut worry_level: i32 = self.items.remove(0);
        worry_level = _perform_operation(&self.operation, worry_level);
        worry_level /= 3;

        self.inspection_counter += 1;

        if worry_level % self.test == 0 {
            (worry_level, self.target_true)
        } else {
            (worry_level, self.target_false)
        }
    }
}

#[test]
fn test_inspect_next() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows(&"data/day11.test.txt".to_string());
    let mut monkeys: Vec<Monkey> = parse_monkeys(&_rows);
    assert_eq!(monkeys[0].inspect_next(), ((79 * 19) / 3, 3));
    assert_eq!(monkeys[0].inspect_next(), ((98 * 19) / 3, 3));
    assert_eq!(monkeys[1].inspect_next(), ((54 + 6) / 3, 0));
}

pub fn parse_monkeys(_rows: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut monkey_strings: Vec<&String> = Vec::new();
    for line in _rows {
        if line.is_empty() {
            monkeys.push(Monkey::new(&monkey_strings));
            monkey_strings = Vec::new();
        } else {
            monkey_strings.push(line);
        }
    }

    monkeys
}

#[test]
fn test_parse_monkeys() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows(&"data/day11.test.txt".to_string());
    let monkeys: Vec<Monkey> = parse_monkeys(&_rows);
    assert_eq!(monkeys[0].items, [79, 98].to_vec());
    assert_eq!(monkeys[1].operation, "new = old + 6");
    assert_eq!(monkeys[2].test, 13);
    assert_eq!(monkeys[3].target_true, 0);
    assert_eq!(monkeys[1].target_false, 0);
}

pub fn run_rounds(_monkeys: &mut Vec<Monkey>, n_rounds: usize) -> i64 {
    for _ in 0..n_rounds {
        for monkey_idx in 0.._monkeys.len() {
            while !_monkeys[monkey_idx].items.is_empty() {
                let inspection_ans: (i32, usize) = _monkeys[monkey_idx].inspect_next();
                _monkeys[inspection_ans.1].items.push(inspection_ans.0);
            }
        }
    }

    let mut inspection_counters: Vec<i64> = Vec::new();
    for monkey in _monkeys {
        inspection_counters.push(monkey.inspection_counter as i64);
    }
    inspection_counters.sort();
    inspection_counters[inspection_counters.len() - 1]
        * inspection_counters[inspection_counters.len() - 2]
}
