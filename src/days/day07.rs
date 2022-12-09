use crate::utils::fs::commands2fs;
use crate::utils::fs::find_smallest_large_folder;
use crate::utils::fs::total_size_small_folders;
use crate::utils::fs::Folder;
use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    assert_eq!(_rows[0], "$ cd /");

    let root: Folder;
    (root, _) = commands2fs(_rows[1..].to_vec(), Folder::new());

    let size_threshold: usize = 100000;
    total_size_small_folders(root, size_threshold) as i32
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day07.test.txt".to_string()), 95437);
}

pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    assert_eq!(_rows[0], "$ cd /");

    let root: Folder;
    (root, _) = commands2fs(_rows[1..].to_vec(), Folder::new());

    let total_disk_space: usize = 70000000;
    let min_free_space: usize = 30000000;
    let current_free_space: usize = total_disk_space - root.size();
    find_smallest_large_folder(root, min_free_space - current_free_space) as i32
}

#[test]
fn test_pt2() {
    assert_eq!(pt2("data/day07.test.txt".to_string()), 24933642);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let small_folders_size: i32 = pt1("data/day07.txt".to_string());
            println!("Total size of small folders: {}", small_folders_size);
            small_folders_size
        }
        2 => {
            let smallest_folder_size: i32 = pt2("data/day07.txt".to_string());
            println!("Size of the folder to delet is: {}", smallest_folder_size);
            smallest_folder_size
        }
        _ => panic!("Part {} not found.", part),
    };
}
