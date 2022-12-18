use crate::utils::graph::find_point;
use crate::utils::graph::hike_downhill;
use crate::utils::graph::hike_uphill;
use crate::utils::graph::map2matrix;
use crate::utils::graph::print_path;
use crate::utils::graph::CellCoords;
use crate::utils::graph::Tree;
use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> usize {
    let _rows: Vec<String> = read_rows(&file_path);
    let matrix: Vec<Vec<char>> = map2matrix(&_rows);
    let tree: Tree;
    (tree, _) = hike_uphill(&matrix, &10000);
    let destination: &CellCoords = &find_point(&matrix, 'E');
    print_path(&matrix, &tree, destination);
    tree.len() - 1
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day12.test.txt".to_string()), 31);
}

pub fn pt2(file_path: String) -> usize {
    let _rows: Vec<String> = read_rows(&file_path);
    let matrix: Vec<Vec<char>> = map2matrix(&_rows);
    let tree: Tree;
    (tree, _) = hike_downhill(&matrix, &10000);
    print_path(
        &matrix,
        &tree,
        &tree.layers[tree.layers.len() - 1][tree.layers[tree.layers.len() - 1].len() - 1].cell,
    );
    tree.len() - 1
}

#[test]
fn test_pt2() {
    assert_eq!(pt2("data/day12.test.txt".to_string()), 29);
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let path_len: usize = pt1("data/day12.txt".to_string());
            println!("Shortest path length: {}", path_len);
            path_len as i32
        }
        2 => {
            let path_len: usize = pt2("data/day12.txt".to_string());
            println!("Shortest path length: {}", path_len);
            path_len as i32
        }
        _ => panic!("Part {} not found.", part),
    };
}
