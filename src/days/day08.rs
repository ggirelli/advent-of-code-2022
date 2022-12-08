use crate::utils::io::read_rows;
use crate::utils::matrix::count_nonzero;
use crate::utils::matrix::filter_visible;
use crate::utils::matrix::text2mtx;

pub fn pt1(file_path: String) -> usize {
    let _rows: Vec<String> = read_rows(file_path);
    let _mtx: Vec<Vec<i32>> = text2mtx(&_rows);
    let visible_mtx: Vec<Vec<i32>> = filter_visible(&_mtx);
    count_nonzero(&visible_mtx)
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day08.test.txt".to_string()), 21);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let visible_trees: usize = pt1("data/day08.txt".to_string());
            println!("Visible tree count: {}", visible_trees);
            visible_trees as i32
        }
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
