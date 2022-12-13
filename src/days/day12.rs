use crate::utils::graph::closest_step;
use crate::utils::graph::explore_map;
use crate::utils::graph::find_all_points;
use crate::utils::graph::find_point;
use crate::utils::graph::map2matrix;
use crate::utils::graph::print_path;
use crate::utils::graph::tree2path;
use crate::utils::graph::CellCoords;
use crate::utils::graph::Tree;
use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> usize {
    let _rows: Vec<String> = read_rows(&file_path);
    let matrix: Vec<Vec<char>> = map2matrix(&_rows);
    let tree: Tree;
    (tree, _) = explore_map(&matrix, &10000);
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
    let mut matrix: Vec<Vec<char>> = map2matrix(&_rows);

    let mut current_start: &CellCoords = &find_point(&matrix, 'S');
    let destination: &CellCoords = &find_point(&matrix, 'E');

    let mut tree: Tree;
    let mut global_visited: Vec<CellCoords> = Vec::new();
    let mut current_visited: Vec<CellCoords>;

    let mut shortest_tree_length: usize = 10000;
    let mut current_tree_length: usize;

    let _start_points: Vec<CellCoords> = find_all_points(&matrix, 'a');
    for start_point in &_start_points {
        if global_visited.contains(start_point) {
            continue;
        }

        matrix[current_start.row][current_start.col] = 'a';
        current_start = start_point;
        matrix[current_start.row][current_start.col] = 'S';

        (tree, current_visited) = explore_map(&matrix, &(shortest_tree_length + 1));
        if !current_visited.contains(destination) {
            continue;
        }

        let path = tree2path(&tree, destination);
        for (step, _) in &path {
            if !global_visited.contains(step) {
                global_visited.push(step.copy());
            }
        }

        current_tree_length = closest_step(&path, &matrix, 'a');
        if current_tree_length >= shortest_tree_length {
            continue;
        }

        print_path(&matrix, &tree, destination);
        shortest_tree_length = current_tree_length;
    }
    shortest_tree_length
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
