use crate::utils::io::{group_rows, read_rows};

fn assert_packet_is_list(packet: &String) {
    assert_eq!(packet.chars().nth(0).unwrap(), '[');
    assert_eq!(packet.chars().nth(packet.len() - 1).unwrap(), ']');
}

#[test]
fn test_assert_packet_is_list() {
    assert_packet_is_list(&String::from("[1,2,3,4]"))
}

#[test]
#[should_panic]
fn test_assert_packet_is_list_fail() {
    assert_packet_is_list(&String::from("[1,2,3,4"))
}

fn extract_next_value(packet: &String) -> (String, String) {
    let mut position: usize = 0;
    if packet.chars().nth(position).unwrap() == '[' {
        assert!(packet[1..].contains(']'));
    } else {
    }
    (String::from(""), String::from(""))
}

fn parse_and_compare_packets(p1: &String, p2: &String) -> bool {
    assert_packet_is_list(p1);
    assert_packet_is_list(p2);
    true
}

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    let packet_pairs: Vec<Vec<String>> = group_rows(&_rows, String::from(""));
    for pair in packet_pairs {
        if pair.len() != 2 {
            panic!("Found {} packets instead of 2.", pair.len());
        }
    }
    0
}

#[test]
fn test_pt1() {}

pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    0
}

#[test]
fn test_pt2() {}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => pt1("data/day12.txt".to_string()),
        2 => pt2("data/day12.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
