use crate::utils::fs::commands2fs;
use crate::utils::fs::find_smallest_large_folder;
use crate::utils::fs::total_size_small_folders;
use crate::utils::fs::Folder;
use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    0
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day08.test.txt".to_string()), 95437);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => pt1("data/day08.txt".to_string()),
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
