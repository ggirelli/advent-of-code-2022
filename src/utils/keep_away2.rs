use crate::utils::modulus::Modulus;

fn _parse_starting_item_values(line: &String) -> Vec<u64> {
    assert_eq!(&line[..18], "  Starting items: ");
    line[18..]
        .split(", ")
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

#[test]
fn test_parse_starting_item_values() {
    assert_eq!(
        _parse_starting_item_values(&"  Starting items: 79, 98".to_string()),
        [79, 98].to_vec()
    );
    assert_eq!(
        _parse_starting_item_values(&"  Starting items: 54, 65, 75, 74".to_string()),
        [54, 65, 75, 74].to_vec()
    );
    assert_eq!(
        _parse_starting_item_values(&"  Starting items: 79, 60, 97".to_string()),
        [79, 60, 97].to_vec()
    );
    assert_eq!(
        _parse_starting_item_values(&"  Starting items: 74".to_string()),
        [74].to_vec()
    );
}

#[test]
#[should_panic]
fn test_parse_starting_item_values_panic() {
    _parse_starting_item_values(&"Starting items: 79, 98".to_string());
}

fn _extract_operation(line: &String) -> String {
    assert_eq!(&line[..13], "  Operation: ");
    line[13..].to_string()
}

#[test]
fn test_extract_operation() {
    assert_eq!(
        _extract_operation(&"  Operation: new = old * 19".to_string()),
        "new = old * 19".to_string()
    );
    assert_eq!(
        _extract_operation(&"  Operation: new = old + 6".to_string()),
        "new = old + 6".to_string()
    );
    assert_eq!(
        _extract_operation(&"  Operation: new = old * old".to_string()),
        "new = old * old".to_string()
    );
    assert_eq!(
        _extract_operation(&"  Operation: new = old + 3".to_string()),
        "new = old + 3".to_string()
    );
}

#[test]
#[should_panic]
fn test_extract_operation_panic() {
    _extract_operation(&"Operation: new = old * 19".to_string());
}

fn _parse_test(line: &String) -> u64 {
    assert_eq!(&line[..21], "  Test: divisible by ");
    line[21..].parse::<u64>().unwrap()
}

#[test]
fn test_parse_test() {
    assert_eq!(_parse_test(&"  Test: divisible by 23".to_string()), 23);
    assert_eq!(_parse_test(&"  Test: divisible by 19".to_string()), 19);
    assert_eq!(_parse_test(&"  Test: divisible by 13".to_string()), 13);
    assert_eq!(_parse_test(&"  Test: divisible by 17".to_string()), 17);
}

#[test]
#[should_panic]
fn test_parse_test_panic() {
    _parse_test(&"Test: divisible by ".to_string());
}

fn _parse_target_true(line: &String) -> usize {
    assert_eq!(&line[..29], "    If true: throw to monkey ");
    line[29..].parse::<usize>().unwrap()
}

fn _parse_target_false(line: &String) -> usize {
    assert_eq!(&line[..30], "    If false: throw to monkey ");
    line[30..].parse::<usize>().unwrap()
}

#[test]
fn test_parse_target() {
    assert_eq!(
        _parse_target_true(&"    If true: throw to monkey 2".to_string()),
        2
    );
    assert_eq!(
        _parse_target_true(&"    If true: throw to monkey 1".to_string()),
        1
    );
    assert_eq!(
        _parse_target_true(&"    If true: throw to monkey 0".to_string()),
        0
    );

    assert_eq!(
        _parse_target_false(&"    If false: throw to monkey 3".to_string()),
        3
    );
    assert_eq!(
        _parse_target_false(&"    If false: throw to monkey 0".to_string()),
        0
    );
    assert_eq!(
        _parse_target_false(&"    If false: throw to monkey 1".to_string()),
        1
    );
}

#[test]
#[should_panic]
fn test_parse_target_true_panic() {
    _parse_target_true(&"   If true: throw to monkey 0".to_string());
}

#[test]
#[should_panic]
fn test_parse_target_false_panic() {
    _parse_target_false(&"   If true: throw to monkey 0".to_string());
}

fn _perform_operation(operation_string: &String, n: &mut Modulus) {
    assert!(operation_string.len() >= 13);

    let value: u64;
    match &operation_string[12..] {
        "old" => value = n.remainder,
        _ => {
            value =
                Modulus::new(&operation_string[12..].parse::<u64>().unwrap(), n.modulus).remainder
        }
    }

    let operator_char: char = operation_string
        .chars()
        .nth(10)
        .expect("ERROR: operation string is too short.");
    match operator_char {
        '*' => n.multiply(value),
        '+' => n.add(value),
        _ => panic!(
            "ERROR: unrecognized '{}' operation. ({})",
            operator_char, operation_string
        ),
    }
}

#[test]
fn test_perform_operation() {
    let mut item: Modulus;
    item = Modulus::new(&1, 5);
    _perform_operation(&"new = old * 19".to_string(), &mut item);
    assert_eq!(item.remainder, 4);

    item = Modulus::new(&3, 5);
    _perform_operation(&"new = old * 19".to_string(), &mut item);
    assert_eq!(item.remainder, 57 % 5);

    item = Modulus::new(&1, 5);
    _perform_operation(&"new = old + 6".to_string(), &mut item);
    assert_eq!(item.remainder, 7 % 5);

    item = Modulus::new(&57, 5);
    _perform_operation(&"new = old + 6".to_string(), &mut item);
    assert_eq!(item.remainder, 63 % 5);

    item = Modulus::new(&1, 5);
    _perform_operation(&"new = old * old".to_string(), &mut item);
    assert_eq!(item.remainder, 1);

    item = Modulus::new(&5, 5);
    _perform_operation(&"new = old * old".to_string(), &mut item);
    assert_eq!(item.remainder, 25 % 5);
}

#[test]
#[should_panic]
fn test_perform_operation_panic() {
    _perform_operation(&"new = old * ".to_string(), &mut Modulus::new(&1, 5));
}

fn values2items(_values: &Vec<u64>, modulus: &u64) -> Vec<Modulus> {
    let mut items: Vec<Modulus> = Vec::new();
    for value in _values {
        items.push(Modulus::new(value, modulus + 0));
    }
    items
}

pub struct Monkey {
    pub items: Vec<Modulus>,
    operation: String,
    pub test: u64,
    target_true: usize,
    target_false: usize,
    pub inspection_counter: usize,
}

impl Monkey {
    pub fn new(_rows: &Vec<&String>, modulus_base: &u64) -> Monkey {
        assert_eq!(_rows.len(), 6);
        let test_value: u64 = _parse_test(&_rows[3]);
        assert_eq!(modulus_base % test_value, 0);
        Monkey {
            items: values2items(&_parse_starting_item_values(&_rows[1]), modulus_base),
            operation: _extract_operation(&_rows[2]),
            test: test_value,
            target_true: _parse_target_true(&_rows[4]),
            target_false: _parse_target_false(&_rows[5]),
            inspection_counter: 0,
        }
    }

    pub fn inspect_next(&mut self) -> (Modulus, usize) {
        assert!(self.items.len() > 0);

        let mut worry_level: Modulus = self.items.remove(0);
        _perform_operation(&self.operation, &mut worry_level);

        self.inspection_counter += 1;

        if (worry_level.remainder % self.test) == 0 {
            return (worry_level, self.target_true);
        } else {
            return (worry_level, self.target_false);
        }
    }
}

#[test]
fn test_inspect_next() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows(&"data/day11.test.txt".to_string());
    let mut monkeys: Vec<Monkey> = parse_monkeys(&_rows, &(3 * 23 * 19 * 13 * 17));
    assert_eq!(
        monkeys[0].inspect_next().0.remainder % monkeys[0].test,
        Modulus::new(&(79 * 19), monkeys[0].test).remainder
    );
    assert_eq!(
        monkeys[0].inspect_next().0.remainder % monkeys[0].test,
        Modulus::new(&(98 * 19), monkeys[0].test).remainder
    );
    assert_eq!(
        monkeys[1].inspect_next().0.remainder % monkeys[1].test,
        Modulus::new(&(54 + 6), monkeys[1].test).remainder
    );
}

pub fn parse_monkeys(_rows: &Vec<String>, modulus_base: &u64) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut monkey_strings: Vec<&String> = Vec::new();
    for line in _rows {
        if line == "" {
            monkeys.push(Monkey::new(&monkey_strings, modulus_base));
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
    let modulus_base = update_modulus_base(&_rows, 1);
    let monkeys: Vec<Monkey> = parse_monkeys(&_rows, &modulus_base);

    let test_items: Vec<Modulus> = vec![Modulus::new(&79, modulus_base),Modulus::new(&98, modulus_base)];

    assert_eq!(monkeys[0].items, test_items);
    assert_eq!(monkeys[1].operation, "new = old + 6");
    assert_eq!(monkeys[2].test, 13);
    assert_eq!(monkeys[3].target_true, 0);
    assert_eq!(monkeys[1].target_false, 0);
}

pub fn update_modulus_base(_rows: &Vec<String>, mut modulus_base: u64) -> u64 {
    for line in _rows {
        if line.len() < 21 {
            continue;
        }
        if line[..21] == "  Test: divisible by ".to_string() {
            modulus_base *= line[21..].parse::<u64>().unwrap();
        }
    }
    modulus_base
}

pub fn run_rounds(_monkeys: &mut Vec<Monkey>, n_rounds: usize) -> i64 {
    for _ in 0..n_rounds {
        for monkey_idx in 0.._monkeys.len() {
            while _monkeys[monkey_idx].items.len() > 0 {
                let inspection_ans: (Modulus, usize) = _monkeys[monkey_idx].inspect_next();

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
