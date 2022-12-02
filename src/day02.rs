//use std::fs;

use crate::io::read_rows;

pub fn pt1(file_path: String) -> i32 {
    let _rows = read_rows(file_path);

    let mut total_score = 0;
    for row in _rows {
        let moves: Vec<&str> = row.split(" ").collect();

        let mut score = 0;

        match moves[1] {
            "X" => score += 1,
            "Y" => score += 2,
            "Z" => score += 3,
            _ => {},
        }

        match [moves[0], moves[1]] {
            ["A", "Y"] => score += 6,
            ["B", "Z"] => score += 6,
            ["C", "X"] => score += 6,
            ["A", "X"] => score += 3,
            ["B", "Y"] => score += 3,
            ["C", "Z"] => score += 3,
            _ => {},
        }

        total_score += score;
    }

    return total_score;
}

#[test]
fn test_pt1() {
    let answer: i32 = pt1("data/day02.test.txt".to_string());
    assert_eq!(answer, 15);
}

pub fn pt2(file_path: String) -> i32 {
    let _rows = read_rows(file_path);

    let mut total_score = 0;
    for row in _rows {
        let moves: Vec<&str> = row.split(" ").collect();

        let mut score = 0;

        match moves[0] {
            "A" => {
                match moves[1] {
                    "X" => score += 0 + 3,
                    "Y" => score += 3 + 1,
                    "Z" => score += 6 + 2,
                    _ => {},
                }
            },
            "B" => {
                match moves[1] {
                    "X" => score += 0 + 1,
                    "Y" => score += 3 + 2,
                    "Z" => score += 6 + 3,
                    _ => {},
                }
            },
            "C" => {
                match moves[1] {
                    "X" => score += 0 + 2,
                    "Y" => score += 3 + 3,
                    "Z" => score += 6 + 1,
                    _ => {},
                }
            },
            _ => {},
        }

        total_score += score;
    }

    return total_score;
}

#[test]
fn test_pt2() {
    let answer: i32 = pt2("data/day02.test.txt".to_string());
    assert_eq!(answer, 12);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let total_score: i32 = pt1("data/day02.txt".to_string());
            println!("Total score: {}", total_score);
            total_score
        },
        2 => {
            let total_score: i32 = pt2("data/day02.txt".to_string());
            println!("Total score: {}", total_score);
            total_score
        },
        _ => {
            println!("Part {} not found.", part);
            0
        },
    };
}