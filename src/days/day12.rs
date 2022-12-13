use crate::utils::graph::closest_step;
use crate::utils::graph::explore_map;
use crate::utils::graph::find_all_points;
use crate::utils::graph::find_point;
use crate::utils::graph::map2matrix;
use crate::utils::graph::print_path;
use crate::utils::graph::tree2path;
use crate::utils::graph::CellCoords;
use crate::utils::io::read_rows;

pub fn pt1(file_path: String) -> usize {
    let _rows: Vec<String> = read_rows(&file_path);
    let matrix: Vec<Vec<char>> = map2matrix(&_rows);
    let tree: Vec<Vec<(usize, CellCoords, char)>>;
    (tree, _) = explore_map(&matrix, &10000);
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

    let mut shortest_tree_length: usize = explore_map(&matrix, &10000).0.len() - 1;
    let mut current_tree_length: usize;

    let _start_points: Vec<CellCoords> = find_all_points(&matrix, 'a');
    for point_idx in 0.._start_points.len() {
        println!(
            "{}/{} ({})",
            point_idx,
            _start_points.len(),
            shortest_tree_length
        );

        matrix[current_start.row][current_start.col] = 'a';
        matrix[_start_points[point_idx].row][_start_points[point_idx].col] = 'S';

        let tree: Vec<Vec<(usize, CellCoords, char)>>;
        let visited: Vec<CellCoords>;
        (tree, visited) = explore_map(&matrix, &(shortest_tree_length + 1));
        if !visited.contains(destination) {
            current_start = &_start_points[point_idx];
            continue;
        }

        let path = tree2path(&tree, destination);
        current_tree_length = closest_step(&path, &matrix, 'a');
        if current_tree_length >= shortest_tree_length {
            current_start = &_start_points[point_idx];
            continue;
        }

        print_path(&matrix, &tree, destination);
        shortest_tree_length = current_tree_length;
        current_start = &_start_points[point_idx];
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
