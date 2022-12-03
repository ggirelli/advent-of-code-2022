use crate::utils::characters::build_letter_value_map;
use crate::utils::io::read_rows;
use crate::utils::strings::get_common_characters;
use crate::utils::strings::halve_string;

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

    println!("The total priority is {}", total_priority);
    total_priority
}

#[test]
fn test_pt1() {
    let answer: i32 = pt1("data/day03.test.txt".to_string());
    assert_eq!(answer, 157);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let priority_sum: i32 = pt1("data/day03.txt".to_string());
            priority_sum
        }
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
