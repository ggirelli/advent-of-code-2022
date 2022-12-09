use crate::utils::geom::calc_dist_two_points;
use crate::utils::io::read_rows;

fn parse_steps(line: &String) -> i32 {
    line[2..].parse::<i32>().unwrap()
}

#[test]
fn test_parse_steps() {
    assert_eq!(parse_steps(&"L 5".to_string()), 5);
    assert_eq!(parse_steps(&"U 13".to_string()), 13);
}

fn update_position(mut position: (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        'L' => {
            position.0 -= 1;
        }
        'R' => {
            position.0 += 1;
        }
        'U' => {
            position.1 += 1;
        }
        'D' => {
            position.1 -= 1;
        }
        _ => panic!("Unrecognized direction '{}'", direction),
    }
    position
}

#[test]
fn test_update_position() {
    let mut position: (i32, i32) = (0, 0);
    position = update_position(position, 'L');
    assert_eq!(position, (-1, 0));
}

pub fn pt1(file_path: String) -> usize {
    let _rows: Vec<String> = read_rows(&file_path);

    let mut visited_positions: Vec<(i32, i32)> = Vec::new();
    let mut tail_position = (0, 0);
    let mut head_position = (0, 0);
    for row in _rows {
        assert!(row.len() >= 3);

        let steps: i32 = parse_steps(&row);
        let direction: char = row.chars().nth(0).expect("Expected a character.");

        for _ in 0..steps {
            head_position = update_position(head_position, direction);

            let head_tail_dist: f32 = calc_dist_two_points(tail_position, head_position);
            if head_tail_dist <= 1.0 {
                if !visited_positions.contains(&tail_position) {
                    visited_positions.push(tail_position);
                }
                continue;
            } else if head_tail_dist == 2.0 {
                tail_position = update_position(tail_position, direction);
            } else if head_tail_dist > (2 as f32).powf(0.5) {
                if head_position.0 - tail_position.0 > 0 {
                    tail_position.0 += 1;
                }
                if head_position.0 - tail_position.0 < 0 {
                    tail_position.0 -= 1;
                }
                if head_position.1 - tail_position.1 > 0 {
                    tail_position.1 += 1;
                }
                if head_position.1 - tail_position.1 < 0 {
                    tail_position.1 -= 1;
                }
            }

            if !visited_positions.contains(&tail_position) {
                visited_positions.push(tail_position);
            }
        }
    }
    visited_positions.len()
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day09.test.txt".to_string()), 13)
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
            let visited_positions: usize = pt1("data/day09.txt".to_string());
            println!("Visited position count: {}", visited_positions);
            visited_positions as i32
        }
        2 => pt2("data/day09.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
