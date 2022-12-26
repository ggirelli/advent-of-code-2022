use crate::utils::io::{group_row_pairs, read_rows};
use std::cmp::Ordering;

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

fn value_is_list(value: &str) -> bool {
    match value.chars().next() {
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
    if second_value.is_empty() && !first_value.is_empty() {
        return Some(false);
    } else if first_value.is_empty() && !second_value.is_empty() {
        return Some(true);
    } else if first_value.is_empty() && second_value.is_empty() {
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

        if let Some(answer) = compare_values(first_next, second_next) {
            return Some(answer);
        }

        if second_value.is_empty() && !first_value.is_empty() {
            return Some(false);
        } else if first_value.is_empty() && !second_value.is_empty() {
            return Some(true);
        } else if first_value.is_empty() && second_value.is_empty() {
            return None;
        }
    }
}

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    let packet_pairs: Vec<(String, String)> = group_row_pairs(&_rows, String::from(""));
    let mut correct_packet_ids_sum: i32 = 0;
    for (packet_id, packet) in packet_pairs.iter().enumerate() {
        if let Some(answer) = compare_values(packet.0.to_string(), packet.1.to_string()) {
            if answer {
                correct_packet_ids_sum += packet_id as i32 + 1;
            }
        }
    }
    correct_packet_ids_sum
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day13.test.txt".to_string()), 13);
}

fn quicksort_packets(_packets: Vec<String>) -> Vec<String> {
    let mut sorted_packets: Vec<String> = Vec::new();

    match _packets.len().cmp(&2) {
        Ordering::Greater => {
            let mut smaller_packets: Vec<String> = Vec::new();
            let mut larger_packets: Vec<String> = Vec::new();
            let pivot: String = _packets[0].to_string();

            for packet in _packets[1..].iter() {
                if let Some(answer) = compare_values(pivot.to_string(), packet.to_string()) {
                    if answer {
                        larger_packets.push(packet.to_string());
                    } else {
                        smaller_packets.push(packet.to_string());
                    }
                }
            }

            smaller_packets = quicksort_packets(smaller_packets);
            larger_packets = quicksort_packets(larger_packets);

            sorted_packets.append(&mut smaller_packets);
            sorted_packets.push(pivot);
            sorted_packets.append(&mut larger_packets);
        }
        Ordering::Equal => {
            if let Some(answer) = compare_values(_packets[0].to_string(), _packets[1].to_string()) {
                if answer {
                    sorted_packets.push(_packets[0].to_string());
                    sorted_packets.push(_packets[1].to_string());
                } else {
                    sorted_packets.push(_packets[1].to_string());
                    sorted_packets.push(_packets[0].to_string());
                }
            }
        }
        Ordering::Less => return _packets,
    }

    sorted_packets
}

pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);

    // Select all packets
    let mut _packets: Vec<String> = Vec::new();
    for row in _rows {
        if !row.is_empty() {
            _packets.push(row);
        }
    }

    // Add divider packets
    _packets.push("[[2]]".to_string());
    _packets.push("[[6]]".to_string());

    // Sort packets
    _packets = quicksort_packets(_packets);

    // Find dividers
    let mut divider_ids: (i32, i32) = (0, 0);
    for (packet_id, packet) in _packets.iter().enumerate() {
        match packet.as_str() {
            "[[2]]" => divider_ids.0 = packet_id as i32 + 1,
            "[[6]]" => divider_ids.1 = packet_id as i32 + 1,
            _ => continue,
        }
    }

    divider_ids.0 * divider_ids.1
}

#[test]
fn test_pt2() {
    assert_eq!(pt2("data/day13.test.txt".to_string()), 140);
}

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
        2 => {
            let decoder_key: i32 = pt2("data/day13.txt".to_string());
            println!("Decoder key: {}", decoder_key);
            decoder_key
        }
        _ => panic!("Part {} not found.", part),
    };
}
