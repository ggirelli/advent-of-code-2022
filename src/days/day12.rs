use crate::utils::{graph::explore_map, graph::CellCoords, io::read_rows};

pub fn pt1(file_path: String) -> usize {
    let _rows: Vec<String> = read_rows(&file_path);
    let tree: Vec<Vec<(usize, CellCoords, char)>> = explore_map(&_rows);
    tree.len() - 1
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day12.test.txt".to_string()), 31);
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
            let path_len: usize = pt1("data/day12.txt".to_string());
            println!("Shortest path length: {}", path_len);
            path_len as i32
        }
        2 => pt2("data/day12.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
