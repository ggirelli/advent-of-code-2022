use crate::utils::vec::any_ge;

fn _test_lines() -> Vec<String> {
    ["303".to_string(), "050".to_string(), "111".to_string()].to_vec()
}

fn _test_mtx() -> Vec<Vec<i32>> {
    [[3, 0, 3].to_vec(), [0, 5, 0].to_vec(), [1, 1, 1].to_vec()].to_vec()
}

pub fn text2mtx(_lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line_idx in 0.._lines.len() {
        matrix.push(Vec::new());
        for digit in _lines[line_idx].chars() {
            matrix[line_idx].push(digit.to_string().parse::<i32>().unwrap());
        }
        assert_eq!(_lines.len(), matrix[line_idx].len());
    }
    matrix
}

#[test]
fn test_text2mtx() {
    assert_eq!(text2mtx(&_test_lines()), _test_mtx());
}

pub fn count_nonzero(matrix: &Vec<Vec<i32>>) -> usize {
    let mut nonzero_count: usize = 0;
    for row in matrix {
        for cell in row {
            if cell != &0 {
                nonzero_count += 1;
            }
        }
    }
    nonzero_count
}

#[test]
fn test_count_nonzero() {
    assert_eq!(count_nonzero(&_test_mtx()), 6);
}

pub fn get_col(matrix: &Vec<Vec<i32>>, col_idx: usize) -> Vec<i32> {
    let mut column: Vec<i32> = Vec::new();
    for row in matrix {
        assert!(col_idx < row.len());
        column.push(row[col_idx]);
    }
    column
}

#[test]
fn test_get_col() {
    assert_eq!(get_col(&_test_mtx(), 0), [3, 0, 1].to_vec());
    assert_eq!(get_col(&_test_mtx(), 1), [0, 5, 1].to_vec());
    assert_eq!(get_col(&_test_mtx(), 2), [3, 0, 1].to_vec());
}

#[test]
#[should_panic]
fn test_get_col_index_error() {
    get_col(&_test_mtx(), 5);
}

pub fn filter_visible(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut visible_mtx: Vec<Vec<i32>> = Vec::new();

    for row_idx in 0..matrix.len() {
        let row: &Vec<i32> = &matrix[row_idx];
        visible_mtx.push(Vec::new());

        for col_idx in 0..row.len() {
            let col: Vec<i32> = get_col(matrix, col_idx);
            let cell: i32 = row[col_idx];

            if row_idx == 0 || row_idx == (matrix.len() - 1) {
                visible_mtx[row_idx].push(1);
                continue;
            }
            if col_idx == 0 || col_idx == (row.len() - 1) {
                visible_mtx[row_idx].push(1);
                continue;
            }

            visible_mtx[row_idx].push(0);
            // Check left
            if !any_ge(&row[..col_idx].to_vec(), cell) {
                visible_mtx[row_idx][col_idx] = 1;
                continue;
            }
            // Check right
            if !any_ge(&row[(col_idx + 1)..].to_vec(), cell) {
                visible_mtx[row_idx][col_idx] = 1;
                continue;
            }
            // Check above
            if !any_ge(&col[..row_idx].to_vec(), cell) {
                visible_mtx[row_idx][col_idx] = 1;
                continue;
            }
            // Check below
            if !any_ge(&col[(row_idx + 1)..].to_vec(), cell) {
                visible_mtx[row_idx][col_idx] = 1;
                continue;
            }
        }
    }
    visible_mtx
}

#[test]
fn test_filter_visible() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows("data/day08.test.txt".to_string());
    let _mtx: Vec<Vec<i32>> = text2mtx(&_rows);
    let visible_mtx: Vec<Vec<i32>> = filter_visible(&_mtx);
    assert_eq!(visible_mtx[1][1], 1);
    assert_eq!(visible_mtx[1][2], 1);
    assert_eq!(visible_mtx[1][3], 0);
    assert_eq!(visible_mtx[2][1], 1);
    assert_eq!(visible_mtx[2][2], 0);
    assert_eq!(visible_mtx[2][3], 1);
    assert_eq!(visible_mtx[3][1], 0);
    assert_eq!(visible_mtx[3][2], 1);
    assert_eq!(visible_mtx[3][3], 0);
}

pub fn get_scenic_score(matrix: &Vec<Vec<i32>>, row_idx: usize, col_idx: usize) -> usize {
    assert!(row_idx < matrix.len());
    let row: &Vec<i32> = &matrix[row_idx];
    let col: &Vec<i32> = &get_col(matrix, col_idx);
    let cell: i32 = matrix[row_idx][col_idx];

    let mut scenic_score: usize = 1;

    // Check left
    let mut partial_score: usize = 0;
    for tree_idx in (0..col_idx).rev() {
        partial_score += 1;
        if row[tree_idx] >= cell {
            break;
        }
    }
    scenic_score *= partial_score;

    // Check right
    partial_score = 0;
    for tree_idx in (col_idx + 1)..row.len() {
        partial_score += 1;
        if row[tree_idx] >= cell {
            break;
        }
    }
    scenic_score *= partial_score;

    // Check above
    partial_score = 0;
    for tree_idx in (0..row_idx).rev() {
        partial_score += 1;
        if col[tree_idx] >= cell {
            break;
        }
    }
    scenic_score *= partial_score;

    // Check below
    partial_score = 0;
    for tree_idx in (row_idx + 1)..row.len() {
        partial_score += 1;
        if col[tree_idx] >= cell {
            break;
        }
    }
    scenic_score *= partial_score;

    scenic_score
}

#[test]
fn test_get_scenic_score() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows("data/day08.test.txt".to_string());
    let _mtx: Vec<Vec<i32>> = text2mtx(&_rows);
    assert_eq!(get_scenic_score(&_mtx, 1, 2), 4);
    assert_eq!(get_scenic_score(&_mtx, 3, 2), 8);
}
