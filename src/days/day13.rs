use crate::utils::io::{group_row_pairs, read_rows};

fn extract_next_value(packet: &str) -> (String, String) {
    let mut open_bracket_counter: u32 = 0;
    let mut position: usize = 0;
    for letter in packet.chars() {
        match letter {
            '[' => open_bracket_counter += 1,
            ']' => open_bracket_counter -= 1,
            ',' => {
                if open_bracket_counter == 0 {
                    break;
                }
            }
            _ => {}
        }
        position += 1;
    }
    if position == packet.len() {
        (packet[..position].to_string(), "".to_string())
    } else {
        (
            packet[..position].to_string(),
            value_to_string_list(packet[(position + 1)..].to_string()),
        )
    }
}

fn value_is_list(value: &String) -> bool {
    match value.chars().nth(0) {
        Some(letter) => letter == '[',
        None => false,
    }
}

fn value_to_string_list(mut value: String) -> String {
    value.insert(0, '[');
    value.insert(value.len(), ']');
    value
}

fn compare_values(mut first_value: String, mut second_value: String) -> Option<bool> {
    if second_value.len() == 0 && first_value.len() > 0 {
        return Some(false);
    } else if first_value.len() == 0 && second_value.len() > 0 {
        return Some(true);
    } else if first_value.len() == 0 && second_value.len() == 0 {
        return None;
    }

    let first_is_list: bool = value_is_list(&first_value);
    let second_is_list: bool = value_is_list(&second_value);

    if first_is_list && !second_is_list {
        second_value = value_to_string_list(second_value);
    } else if !first_is_list && second_is_list {
        first_value = value_to_string_list(first_value);
    } else if !first_is_list && !second_is_list {
        let first_value: i32 = first_value.parse::<i32>().unwrap();
        let second_value: i32 = second_value.parse::<i32>().unwrap();
        if first_value == second_value {
            return None;
        } else {
            return Some(first_value < second_value);
        }
    }

    let mut first_next: String;
    let mut second_next: String;
    loop {
        (first_next, first_value) = extract_next_value(&first_value[1..(first_value.len() - 1)]);
        (second_next, second_value) =
            extract_next_value(&second_value[1..(second_value.len() - 1)]);

        match compare_values(first_next, second_next) {
            Some(answer) => return Some(answer),
            None => {}
        }

        if second_value.len() == 0 && first_value.len() > 0 {
            return Some(false);
        } else if first_value.len() == 0 && second_value.len() > 0 {
            return Some(true);
        } else if first_value.len() == 0 && second_value.len() == 0 {
            return None;
        }
    }
}

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    let packet_pairs: Vec<(String, String)> = group_row_pairs(&_rows, String::from(""));
    let mut correct_packet_ids_sum: i32 = 0;
    for (packet_id, packet) in packet_pairs.iter().enumerate() {
        match compare_values(packet.0.to_string(), packet.1.to_string()) {
            Some(answer) => {
                if answer {
                    correct_packet_ids_sum += packet_id as i32 + 1;
                }
            }
            None => {}
        }
    }
    correct_packet_ids_sum
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day13.test.txt".to_string()), 13);
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
            let total_correct_packet_ids: i32 = pt1("data/day13.txt".to_string());
            println!(
                "Total sum of correct packet IDs: {}",
                total_correct_packet_ids
            );
            total_correct_packet_ids
        }
        2 => pt2("data/day13.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
