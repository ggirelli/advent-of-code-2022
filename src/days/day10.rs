use crate::utils::io::read_rows;

fn extract_values(line: &String) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    values.push(0);

    match &line[..4] {
        "addx" => values.push(line[5..].parse::<i32>().unwrap()),
        "noop" => {}
        _ => panic!("ERROR: unrecognized instruction."),
    }

    values
}

#[test]
fn test_extract_value() {
    assert_eq!(extract_values(&"noop".to_string()), [0].to_vec());
    assert_eq!(extract_values(&"addx 15".to_string()), [0, 15].to_vec());
    assert_eq!(extract_values(&"addx -120".to_string()), [0, -120].to_vec());
}

#[test]
#[should_panic]
fn test_extract_value_unrecognized_instruction() {
    extract_values(&"anythingelse".to_string());
}

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);

    let selected_cycles: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut selected_cycles_strength: Vec<i32> = Vec::new();

    let mut cycle_idx: usize = 1;
    let mut register_value: i32 = 1;
    for line in _rows {
        for value in extract_values(&line) {
            cycle_idx += 1;
            register_value += value;

            if selected_cycles.contains(&cycle_idx) {
                selected_cycles_strength.push((cycle_idx as i32) * register_value);
            }
        }
    }

    let mut total_selected_strengths: i32 = 0;
    for strength in selected_cycles_strength {
        total_selected_strengths += strength;
    }
    total_selected_strengths
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day10.test.txt".to_string()), 13140)
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
            let total_selected_strengths: i32 = pt1("data/day10.txt".to_string());
            println!("Sum ot six signal strenghts: {}", total_selected_strengths);
            total_selected_strengths
        }
        2 => pt2("data/day10.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
