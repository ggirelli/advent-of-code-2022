use crate::utils::fs::commands2fs;
use crate::utils::fs::total_size_small_folders;
use crate::utils::fs::Folder;
use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    assert_eq!(_rows[0], "$ cd /");

    let root: Folder;
    (root, _) = commands2fs(_rows[1..].to_vec(), Folder::new());

    let size_threshold: usize = 100000;
    total_size_small_folders(root, size_threshold) as i32
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day07.test.txt".to_string()), 95437)
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let small_folders_size: i32 = pt1("data/day07.txt".to_string());
            println!("Total size of small folders: {}", small_folders_size);
            small_folders_size
        }
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
