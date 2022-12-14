use crate::utils::matrix;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CellCoords {
    pub row: usize,
    pub col: usize,
}

impl CellCoords {
    pub fn copy(&self) -> CellCoords {
        CellCoords {
            row: self.row,
            col: self.col,
        }
    }
}

impl PartialEq for CellCoords {
    fn eq(&self, other: &Self) -> bool {
        (self.row == other.row) & (self.col == other.col)
    }
}

impl fmt::Display for CellCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({};{})", self.row, self.col)
    }
}

fn _test_map() -> Vec<String> {
    ["abc".to_string(), "def".to_string()].to_vec()
}

fn _test_map2() -> Vec<String> {
    [
        "aaaaa".to_string(),
        "abcba".to_string(),
        "abdba".to_string(),
        "abcba".to_string(),
        "aaaaa".to_string(),
    ]
    .to_vec()
}

pub fn map2matrix(_rows: &Vec<String>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for row_idx in 0.._rows.len() {
        matrix.push(Vec::new());
        for letter in _rows[row_idx].chars() {
            matrix[row_idx].push(letter);
        }
        if row_idx > 0 {
            assert_eq!(matrix[row_idx].len(), matrix[row_idx - 1].len())
        }
    }
    matrix
}

#[test]
fn test_map2matrix() {
    assert_eq!(
        map2matrix(&_test_map()),
        [['a', 'b', 'c'].to_vec(), ['d', 'e', 'f'].to_vec()].to_vec()
    );
}

fn map_matrix2heights(matrix: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut heights: Vec<Vec<i32>> = Vec::new();
    for row_idx in 0..matrix.len() {
        heights.push(Vec::new());
        for col_idx in 0..matrix[row_idx].len() {
            heights[row_idx].push(matrix[row_idx][col_idx] as i32);
        }
    }
    heights
}

#[test]
fn test_map_matrix2heights() {
    let matrix: Vec<Vec<char>> = map2matrix(&_test_map());
    assert_eq!(
        map_matrix2heights(&matrix),
        [[97, 98, 99].to_vec(), [100, 101, 102].to_vec()].to_vec()
    );
}

pub fn find_point(matrix: &[Vec<char>], needle: char) -> CellCoords {
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, item) in row.iter().enumerate() {
            if item == &needle {
                return CellCoords {
                    row: row_idx,
                    col: col_idx,
                };
            }
        }
    }
    panic!("ERROR: needle '{}' not found.", needle);
}

#[test]
fn test_find_point() {
    let matrix: Vec<Vec<char>> = map2matrix(&_test_map());
    assert_eq!(find_point(&matrix, 'a'), CellCoords { row: 0, col: 0 });
    assert_eq!(find_point(&matrix, 'c'), CellCoords { row: 0, col: 2 });
    assert_eq!(find_point(&matrix, 'e'), CellCoords { row: 1, col: 1 });
}

#[test]
#[should_panic]
fn test_find_point_panic() {
    find_point(&map2matrix(&_test_map()), 'g');
}

fn find_neighbors_uphill(heights: &Vec<Vec<i32>>, src: &CellCoords) -> Vec<(CellCoords, char)> {
    let mut neighbors: Vec<(CellCoords, char)> = Vec::new();
    if heights.is_empty() {
        return neighbors;
    }
    let src_value: i32 = heights[src.row][src.col];
    if (src.row > 0) && (heights[src.row - 1][src.col] <= (src_value + 1)) {
        neighbors.push((
            CellCoords {
                row: src.row - 1,
                col: src.col,
            },
            '^',
        ));
    }
    if (src.row < (heights.len() - 1)) && (heights[src.row + 1][src.col] <= (src_value + 1)) {
        neighbors.push((
            CellCoords {
                row: src.row + 1,
                col: src.col,
            },
            'v',
        ));
    }
    if (src.col > 0) && (heights[src.row][src.col - 1] <= (src_value + 1)) {
        neighbors.push((
            CellCoords {
                row: src.row,
                col: src.col - 1,
            },
            '<',
        ));
    }
    if (src.col < (heights[0].len() - 1)) && (heights[src.row][src.col + 1] <= (src_value + 1)) {
        neighbors.push((
            CellCoords {
                row: src.row,
                col: src.col + 1,
            },
            '>',
        ));
    }
    neighbors
}

#[test]
fn test_find_neighbors_uphill() {
    let matrix: Vec<Vec<char>> = map2matrix(&_test_map());
    let heights: Vec<Vec<i32>> = map_matrix2heights(&matrix);
    assert_eq!(
        find_neighbors_uphill(&heights, &CellCoords { row: 0, col: 1 }),
        [
            (CellCoords { row: 0, col: 0 }, '<'),
            (CellCoords { row: 0, col: 2 }, '>')
        ]
        .to_vec()
    );
}

fn find_neighbors_downhill(heights: &Vec<Vec<i32>>, src: &CellCoords) -> Vec<(CellCoords, char)> {
    let mut neighbors: Vec<(CellCoords, char)> = Vec::new();
    if heights.is_empty() {
        return neighbors;
    }
    let src_value: i32 = heights[src.row][src.col];
    if (src.row > 0) && (heights[src.row - 1][src.col] - src_value > -2) {
        neighbors.push((
            CellCoords {
                row: src.row - 1,
                col: src.col,
            },
            '^',
        ));
    }
    if (src.row < (heights.len() - 1)) && (heights[src.row + 1][src.col] - src_value > -2) {
        neighbors.push((
            CellCoords {
                row: src.row + 1,
                col: src.col,
            },
            'v',
        ));
    }
    if (src.col > 0) && (heights[src.row][src.col - 1] - src_value > -2) {
        neighbors.push((
            CellCoords {
                row: src.row,
                col: src.col - 1,
            },
            '<',
        ));
    }
    if (src.col < (heights[0].len() - 1)) && (heights[src.row][src.col + 1] - src_value > -2) {
        neighbors.push((
            CellCoords {
                row: src.row,
                col: src.col + 1,
            },
            '>',
        ));
    }
    neighbors
}

#[test]
fn test_find_neighbors_downhill() {
    let matrix: Vec<Vec<char>> = map2matrix(&_test_map2());
    let heights: Vec<Vec<i32>> = map_matrix2heights(&matrix);
    assert_eq!(
        find_neighbors_downhill(&heights, &CellCoords { row: 2, col: 2 }),
        vec![
            (CellCoords { row: 1, col: 2 }, '^'),
            (CellCoords { row: 3, col: 2 }, 'v')
        ]
    );
}

#[derive(Debug)]
pub struct TreeStep {
    parent_idx: usize,
    pub cell: CellCoords,
    from_direction: char,
}

impl fmt::Display for TreeStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{} {} {}",
            self.parent_idx, self.from_direction, self.cell
        )
    }
}

pub struct Tree {
    pub layers: Vec<Vec<TreeStep>>,
}

impl Tree {
    pub fn len(&self) -> usize {
        self.layers.len()
    }
}

fn bread_first_search_uphill(
    heights: &Vec<Vec<i32>>,
    visited: &mut Vec<CellCoords>,
    src: &CellCoords,
    dst: &CellCoords,
    max_steps: &usize,
) -> Tree {
    let mut tree: Tree = Tree { layers: Vec::new() };
    let mut step_counter: usize = 0;

    // Add starting point
    tree.layers.push(vec![TreeStep {
        parent_idx: 0,
        cell: src.copy(),
        from_direction: 'S',
    }]);
    visited.push(src.copy());

    while (!visited.contains(dst)) & (&step_counter <= max_steps) {
        if step_counter % 10 == 0 {
            println!("Step {}", step_counter);
        }
        let mut new_tree_layer: Vec<TreeStep> = Vec::new();
        for parent_idx in 0..tree.layers[tree.len() - 1].len() {
            let parent_ref: &CellCoords = &tree.layers[tree.len() - 1][parent_idx].cell;
            for (neighbor, direction) in find_neighbors_uphill(heights, parent_ref) {
                if !visited.contains(&neighbor) {
                    visited.push(neighbor.copy());
                    new_tree_layer.push(TreeStep {
                        parent_idx,
                        cell: neighbor.copy(),
                        from_direction: direction,
                    });
                }
            }
        }
        tree.layers.push(new_tree_layer);
        step_counter += 1;
    }
    tree
}

pub fn hike_uphill(matrix: &Vec<Vec<char>>, max_steps: &usize) -> (Tree, Vec<CellCoords>) {
    let mut heights: Vec<Vec<i32>> = map_matrix2heights(matrix);

    let source: CellCoords = find_point(matrix, 'S');
    let destination: CellCoords = find_point(matrix, 'E');

    // Set source point to maximum height to allow hike to start
    heights[source.row][source.col] = 'a' as i32;
    heights[destination.row][destination.col] = matrix::max(&heights, 'z' as i32);

    let mut visited: Vec<CellCoords> = Vec::new();
    let tree: Tree =
        bread_first_search_uphill(&heights, &mut visited, &source, &destination, max_steps);

    //print_path(&matrix, &tree, &destination);
    (tree, visited)
}

#[test]
fn test_hike_uphill() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows(&"data/day12.test.txt".to_string());
    let matrix: Vec<Vec<char>> = map2matrix(&_rows);
    let tree: Tree;
    (tree, _) = hike_uphill(&matrix, &10000);
    assert_eq!(tree.len() - 1, 31);
}

fn bread_first_search_downhill(
    heights: &Vec<Vec<i32>>,
    visited: &mut Vec<CellCoords>,
    src: &CellCoords,
    max_steps: &usize,
) -> (Tree, bool) {
    let mut tree: Tree = Tree { layers: Vec::new() };
    let mut step_counter: usize = 0;

    // Add starting point
    tree.layers.push(vec![TreeStep {
        parent_idx: 0,
        cell: src.copy(),
        from_direction: 'S',
    }]);
    visited.push(src.copy());

    let mut reached_target: bool = false;
    while (!reached_target)
        && (visited.len() < heights.len() * heights[0].len())
        && (&step_counter <= max_steps)
    {
        if step_counter % 10 == 0 {
            println!("Step {}", step_counter);
        }
        let mut new_tree_layer: Vec<TreeStep> = Vec::new();
        for parent_idx in 0..tree.layers[tree.len() - 1].len() {
            let parent_ref: &CellCoords = &tree.layers[tree.len() - 1][parent_idx].cell;
            for (neighbor, direction) in find_neighbors_downhill(heights, parent_ref) {
                if !visited.contains(&neighbor) {
                    visited.push(neighbor.copy());
                    new_tree_layer.push(TreeStep {
                        parent_idx,
                        cell: neighbor.copy(),
                        from_direction: direction,
                    });
                }
                if heights[neighbor.row][neighbor.col] == ('a' as i32) {
                    reached_target = true;
                }
            }
        }
        tree.layers.push(new_tree_layer);
        step_counter += 1;
    }

    (tree, reached_target)
}

pub fn hike_downhill(matrix: &Vec<Vec<char>>, max_steps: &usize) -> (Tree, Vec<CellCoords>) {
    let mut heights: Vec<Vec<i32>> = map_matrix2heights(matrix);

    let source: CellCoords = find_point(matrix, 'S');
    let destination: CellCoords = find_point(matrix, 'E');

    // Set source point to maximum height to allow hike to start
    heights[source.row][source.col] = 'a' as i32;
    heights[destination.row][destination.col] = matrix::max(&heights, 'z' as i32);

    let mut visited: Vec<CellCoords> = Vec::new();
    let tree: Tree = bread_first_search_downhill(&heights, &mut visited, &destination, max_steps).0;

    (tree, visited)
}

pub fn tree2path(tree: &Tree, dst: &CellCoords) -> Vec<(CellCoords, char)> {
    let mut path: Vec<(CellCoords, char)> = [].to_vec();

    let mut parent_idx: i32 = -1;
    for cell_idx in 0..tree.layers[tree.len() - 1].len() {
        if &tree.layers[tree.len() - 1][cell_idx].cell == dst {
            parent_idx = cell_idx as i32;
        }
    }
    if parent_idx == -1 {
        panic!("Destination not found in the end of the tree");
    }

    let mut direction: char = 'E';
    for layer_idx in (0..tree.len()).rev() {
        path.insert(
            0,
            (
                tree.layers[layer_idx][parent_idx as usize].cell.copy(),
                direction,
            ),
        );
        direction = tree.layers[layer_idx][parent_idx as usize].from_direction;
        parent_idx = tree.layers[layer_idx][parent_idx as usize].parent_idx as i32;
    }

    path
}

pub fn print_path(matrix: &Vec<Vec<char>>, tree: &Tree, dst: &CellCoords) {
    let mut map_vec: Vec<Vec<char>> = Vec::new();
    for row_idx in 0..matrix.len() {
        map_vec.push(Vec::new());
        for _ in 0..matrix[row_idx].len() {
            map_vec[row_idx].push('.');
        }
    }

    map_vec[dst.row][dst.col] = 'E';

    let path: Vec<(CellCoords, char)> = tree2path(tree, dst);
    for (step, direction) in path {
        map_vec[step.row][step.col] = direction;
    }

    let mut map_string: String = String::new();
    for row in map_vec {
        for pixel in row {
            map_string.push(pixel);
        }
        map_string.push('\n');
    }
    map_string.pop();
    println!("\n{}\n", map_string);
}
