use crate::utils::io::read_rows;
use crate::utils::matrix::count_nonzero;
use crate::utils::matrix::filter_visible;
use crate::utils::matrix::get_scenic_score;
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

pub fn pt2(file_path: String) -> usize {
    let _rows: Vec<String> = read_rows(file_path);
    let _mtx: Vec<Vec<i32>> = text2mtx(&_rows);

    let mut best_scenic_score: usize = 0;
    for row_idx in 0.._mtx.len() {
        for col_idx in 0.._mtx[row_idx].len() {
            let scenic_score: usize = get_scenic_score(&_mtx, row_idx, col_idx);
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }
    best_scenic_score
}

#[test]
fn test_pt2() {
    assert_eq!(pt2("data/day08.test.txt".to_string()), 8);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let visible_trees: usize = pt1("data/day08.txt".to_string());
            println!("Visible tree count: {}", visible_trees);
            visible_trees as i32
        }
        2 => {
            let scenic_score: usize = pt2("data/day08.txt".to_string());
            println!("Best scenic score: {}", scenic_score);
            scenic_score as i32
        }
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
