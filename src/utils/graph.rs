use crate::utils::matrix;

#[derive(Debug, Clone)]
pub struct CellCoords {
    pub row: usize,
    pub col: usize,
}

impl CellCoords {
    pub fn copy(&self) -> CellCoords {
        CellCoords {
            row: self.row + 0,
            col: self.col + 0,
        }
    }
}

impl PartialEq for CellCoords {
    fn eq(&self, other: &Self) -> bool {
        (self.row == other.row) & (self.col == other.col)
    }
}

fn _test_map() -> Vec<String> {
    ["abc".to_string(), "def".to_string()].to_vec()
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

pub fn find_point(matrix: &Vec<Vec<char>>, needle: char) -> CellCoords {
    for row_idx in 0..matrix.len() {
        for col_idx in 0..matrix[row_idx].len() {
            if matrix[row_idx][col_idx] == needle {
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

pub fn find_all_points(matrix: &Vec<Vec<char>>, needle: char) -> Vec<CellCoords> {
    let mut points: Vec<CellCoords> = Vec::new();
    for row_idx in 0..matrix.len() {
        for col_idx in 0..matrix[row_idx].len() {
            if matrix[row_idx][col_idx] == needle {
                points.push(CellCoords {
                    row: row_idx,
                    col: col_idx,
                });
            }
        }
    }
    points
}

#[test]
fn test_find_all_points() {
    let matrix: Vec<Vec<char>> = map2matrix(&_test_map());
    assert_eq!(
        find_all_points(&matrix, 'a'),
        [CellCoords { row: 0, col: 0 }].to_vec()
    );
    assert_eq!(
        find_all_points(&matrix, 'c'),
        [CellCoords { row: 0, col: 2 }].to_vec()
    );
    assert_eq!(
        find_all_points(&matrix, 'e'),
        [CellCoords { row: 1, col: 1 }].to_vec()
    );
}

#[test]
#[should_panic]
fn test_find_point_panic() {
    find_point(&map2matrix(&_test_map()), 'g');
}

fn find_neighbors(heights: &Vec<Vec<i32>>, src: &CellCoords) -> Vec<(CellCoords, char)> {
    let mut neighbors: Vec<(CellCoords, char)> = Vec::new();
    if heights.len() == 0 {
        return neighbors;
    }
    let src_value: i32 = heights[src.row][src.col];
    if src.row > 0 {
        if heights[src.row - 1][src.col] <= (src_value + 1) {
            neighbors.push((
                CellCoords {
                    row: src.row - 1,
                    col: src.col,
                },
                '^',
            ));
        }
    }
    if src.row < (heights.len() - 1) {
        if heights[src.row + 1][src.col] <= (src_value + 1) {
            neighbors.push((
                CellCoords {
                    row: src.row + 1,
                    col: src.col,
                },
                'v',
            ));
        }
    }
    if src.col > 0 {
        if heights[src.row][src.col - 1] <= (src_value + 1) {
            neighbors.push((
                CellCoords {
                    row: src.row,
                    col: src.col - 1,
                },
                '<',
            ));
        }
    }
    if src.col < (heights[0].len() - 1) {
        if heights[src.row][src.col + 1] <= (src_value + 1) {
            neighbors.push((
                CellCoords {
                    row: src.row,
                    col: src.col + 1,
                },
                '>',
            ));
        }
    }
    neighbors
}

#[test]
fn test_find_neighbors() {
    let matrix: Vec<Vec<char>> = map2matrix(&_test_map());
    let heights: Vec<Vec<i32>> = map_matrix2heights(&matrix);
    assert_eq!(
        find_neighbors(&heights, &CellCoords { row: 0, col: 1 }),
        [
            (CellCoords { row: 0, col: 0 }, '<'),
            (CellCoords { row: 0, col: 2 }, '>')
        ]
        .to_vec()
    );
}

fn bread_first_search_heights(
    heights: &Vec<Vec<i32>>,
    visited: &mut Vec<CellCoords>,
    src: &CellCoords,
    dst: &CellCoords,
    max_steps: &usize,
) -> Vec<Vec<(usize, CellCoords, char)>> {
    let mut tree: Vec<Vec<(usize, CellCoords, char)>> = Vec::new();
    let mut step_counter: usize = 0;

    // Add starting point
    tree.push([(0, (&src).copy(), 'S')].to_vec());
    visited.push((&src).copy());

    while (!visited.contains(&&dst)) & (&step_counter <= max_steps) {
        let mut new_tree_layer: Vec<(usize, CellCoords, char)> = Vec::new();
        for parent_idx in 0..tree[tree.len() - 1].len() {
            let parent_ref: &CellCoords = &tree[tree.len() - 1][parent_idx].1;
            for (neighbor, direction) in find_neighbors(&heights, parent_ref) {
                if !visited.contains(&neighbor) {
                    visited.push((&neighbor).copy());
                    new_tree_layer.push((parent_idx, (&neighbor).copy(), direction));
                }
            }
        }
        tree.push(new_tree_layer);
        step_counter += 1;
    }
    tree
}

pub fn explore_map(
    matrix: &Vec<Vec<char>>,
    max_steps: &usize,
) -> (Vec<Vec<(usize, CellCoords, char)>>, Vec<CellCoords>) {
    let mut heights: Vec<Vec<i32>> = map_matrix2heights(&matrix);

    let source: CellCoords = find_point(&matrix, 'S');
    let destination: CellCoords = find_point(&matrix, 'E');

    // Set source point to maximum height to allow hike to start
    heights[source.row][source.col] = matrix::max(&heights, 'z' as i32);
    heights[destination.row][destination.col] = matrix::max(&heights, 'z' as i32);

    let mut visited: Vec<CellCoords> = Vec::new();
    let tree: Vec<Vec<(usize, CellCoords, char)>> =
        bread_first_search_heights(&heights, &mut visited, &source, &destination, &max_steps);

    //print_path(&matrix, &tree, &destination);
    (tree, visited)
}

pub fn tree2path(
    tree: &Vec<Vec<(usize, CellCoords, char)>>,
    dst: &CellCoords,
) -> Vec<(CellCoords, char)> {
    let mut path: Vec<(CellCoords, char)> = [(dst.copy(), 'E')].to_vec();

    let mut parent_idx: usize = 0;
    for cell_idx in 0..tree[tree.len() - 1].len() {
        if &tree[tree.len() - 1][cell_idx].1 == dst {
            parent_idx = cell_idx;
        }
    }

    let mut direction: char = 'E';
    for layer_idx in (0..tree.len()).rev() {
        path.insert(0, (tree[layer_idx][parent_idx].1.copy(), direction));
        direction = tree[layer_idx][parent_idx].2;
        parent_idx = tree[layer_idx][parent_idx].0;
    }

    path
}

pub fn closest_step(
    path: &Vec<(CellCoords, char)>,
    matrix: &Vec<Vec<char>>,
    needle: char,
) -> usize {
    for step_idx in (0..path.len()).rev() {
        if matrix[path[step_idx].0.row][path[step_idx].0.col] == needle {
            return path.len() - step_idx - 1;
        }
    }
    path.len() - 1
}

pub fn print_path(
    matrix: &Vec<Vec<char>>,
    tree: &Vec<Vec<(usize, CellCoords, char)>>,
    dst: &CellCoords,
) {
    let mut map_vec: Vec<Vec<char>> = Vec::new();
    for row_idx in 0..matrix.len() {
        map_vec.push(Vec::new());
        for _ in 0..matrix[row_idx].len() {
            map_vec[row_idx].push('.');
        }
    }

    map_vec[dst.row][dst.col] = 'E';

    let path: Vec<(CellCoords, char)> = tree2path(&tree, dst);
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
    println!("{}", map_string);
}

#[test]
fn test_explore_map() {
    use crate::utils::io::read_rows;
    let _rows: Vec<String> = read_rows(&"data/day12.test.txt".to_string());
    let matrix: Vec<Vec<char>> = map2matrix(&_rows);
    let tree: Vec<Vec<(usize, CellCoords, char)>>;
    (tree, _) = explore_map(&matrix, &10000);
    assert_eq!(tree.len() - 1, 31);
}
