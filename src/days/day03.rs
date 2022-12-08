use std::collections::HashMap;

use crate::utils::char::build_letter_value_map;
use crate::utils::io::read_rows;
use crate::utils::string::count_characters;
use crate::utils::string::get_common_characters;
use crate::utils::string::halve_string;
use crate::utils::string::unique_string;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    let _letter_values = build_letter_value_map();

    let mut total_priority: i32 = 0;
    for row in _rows {
        let halves: [String; 2] = halve_string(row);
        let common_letters: Vec<char> =
            get_common_characters(halves[0].to_string(), halves[1].to_string());

        for letter in common_letters {
            match _letter_values.get(&letter) {
                Some(&value) => {
                    total_priority += value;
                }
                _ => println!("WARNING: cannot find character '{}'. Skipped!", letter),
            }
        }
    }

    total_priority
}

#[test]
fn test_pt1() {
    let answer: i32 = pt1("data/day03.test.txt".to_string());
    assert_eq!(answer, 157);
}

pub fn pt2(file_path: String) -> i32 {
    let elf_group_size: usize = 3;
    let _rows: Vec<String> = read_rows(file_path);
    let _letter_values = build_letter_value_map();

    let mut total_badge_priority: i32 = 0;

    for elf_id in 0.._rows.len() {
        if elf_id % elf_group_size == 0 {
            let mut elf_group_string: String = String::new();
            for i in 0..3 {
                let elf_items: String = unique_string(_rows[elf_id + i].to_string());
                elf_group_string.push_str(&elf_items.to_string());
            }

            let item_counts: HashMap<char, i32> = count_characters(elf_group_string);
            for (letter, count) in item_counts {
                if count == (elf_group_size as i32) {
                    match _letter_values.get(&letter) {
                        Some(value) => total_badge_priority += value,
                        _ => println!("WARNING: cannot find character '{}'. Skipped!", letter),
                    }
                }
            }
        }
    }

    total_badge_priority
}

#[test]
fn test_pt2() {
    let answer: i32 = pt2("data/day03.test.txt".to_string());
    assert_eq!(answer, 70);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let total_priority: i32 = pt1("data/day03.txt".to_string());
            println!("The total priority is {}", total_priority);
            total_priority
        }
        2 => {
            let total_priority: i32 = pt2("data/day03.txt".to_string());
            println!("The total priority is {}", total_priority);
            total_priority
        }
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
