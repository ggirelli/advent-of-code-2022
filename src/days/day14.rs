use crate::utils::falling_sand::{drop_sand, print_map, scan_to_map, Point};
use crate::utils::io::read_rows;

const SAND_ORIGIN: Point = Point { row: 0, col: 500 };

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);

    let mut _map: Vec<Vec<usize>>;
    let col_translation: usize;
    (_map, col_translation) = scan_to_map(&_rows);

    let mut sand_dropped: usize = 0;
    loop {
        print_map(&_map);
        if !drop_sand(
            &mut _map,
            Point {
                row: SAND_ORIGIN.row,
                col: SAND_ORIGIN.col - col_translation,
            },
        ) {
            break;
        }
        sand_dropped += 1;
    }

    sand_dropped as i32
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day14.test.txt".to_string()), 24);
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
            let sand_units_to_rest: i32 = pt1("data/day14.txt".to_string());
            println!("Total sand units before void: {}", sand_units_to_rest);
            sand_units_to_rest
        }
        2 => pt2("data/day14.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
