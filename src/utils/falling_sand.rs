use crate::utils::matrix::zeros;
use std::fmt;

#[derive(Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.row == other.row) & (self.col == other.col)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({};{})", self.row, self.col)
    }
}

impl Point {
    pub fn from(line: &str) -> Point {
        let coords: Vec<String> = line.split(',').into_iter().map(String::from).collect();
        assert_eq!(coords.len(), 2, "expected 2D coordinates");
        Point {
            col: coords[0].parse::<usize>().expect("col coord not found"),
            row: coords[1].parse::<usize>().expect("row coord not found"),
        }
    }
}

#[test]
fn test_point_from() {
    assert_eq!(Point::from("234,23"), Point { col: 234, row: 23 });
}

fn row_to_points(row: &str) -> ScanTrace {
    let mut scan: ScanTrace = ScanTrace { points: Vec::new() };
    for coords in row.split(" -> ") {
        scan.points.push(Point::from(coords));
    }
    scan
}

#[test]
fn test_row_to_points() {
    assert_eq!(
        row_to_points("234,23 -> 543,2"),
        ScanTrace {
            points: vec![Point { col: 234, row: 23 }, Point { col: 543, row: 2 }]
        }
    );
}

#[derive(Debug)]
struct ScanTrace {
    points: Vec<Point>,
}

impl PartialEq for ScanTrace {
    fn eq(&self, other: &Self) -> bool {
        if self.points.len() != other.points.len() {
            return false;
        }
        for (i, point) in self.points.iter().enumerate() {
            if point != &other.points[i] {
                return false;
            }
        }
        true
    }
}

fn update_lims(lim: &mut (usize, usize), value: usize) {
    if value < lim.0 {
        lim.0 = value;
    }
    if value > lim.1 {
        lim.1 = value;
    }
}

enum Segment {
    Horizontal { y: usize, x1: usize, x2: usize },
    Vertical { x: usize, y1: usize, y2: usize },
}

impl Segment {
    fn from(p1: &Point, p2: &Point) -> Segment {
        if p1.col == p2.col {
            Segment::Horizontal {
                y: p1.col,
                x1: if p1.row < p2.row { p1.row } else { p2.row },
                x2: if p1.row < p2.row { p2.row } else { p1.row },
            }
        } else if p1.row == p2.row {
            Segment::Vertical {
                x: p1.row,
                y1: if p1.col < p2.col { p1.col } else { p2.col },
                y2: if p1.col < p2.col { p2.col } else { p1.col },
            }
        } else {
            panic!("Unsupported non-orthogonal segment: {},{}.", p1, p2);
        }
    }
}

fn draw_segment(map: &mut [Vec<usize>], segment: Segment) {
    match segment {
        Segment::Vertical { x, y1, y2 } => {
            for y in y1..=y2 {
                map[x][y] = 1;
            }
        }
        Segment::Horizontal { y, x1, x2 } => {
            for map_x in map.iter_mut().take(x2 + 1).skip(x1) {
                map_x[y] = 1;
            }
        }
    }
}

pub fn print_map(map: &Vec<Vec<usize>>) {
    for row in map {
        for cell in row {
            match cell {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("o"),
                _ => panic!("Unrecognized map value: {}", cell),
            }
        }
        println!();
    }
}

pub fn scan_to_map(_rows: &Vec<String>, floor: bool) -> (Vec<Vec<usize>>, usize) {
    let mut row_lims: (usize, usize) = (usize::MAX, 0);
    let mut col_lims: (usize, usize) = (usize::MAX, 0);

    let mut _scans: Vec<ScanTrace> = Vec::new();
    for row in _rows {
        _scans.push(row_to_points(row));
        for point in &_scans[_scans.len() - 1].points {
            update_lims(&mut row_lims, point.row);
            update_lims(&mut col_lims, point.col);
        }
    }

    let mut map: Vec<Vec<usize>>;
    let mut col_translation: usize = col_lims.0;
    if floor {
        row_lims.1 += 2;
        col_lims.1 += row_lims.1 * 2;
        col_translation -= row_lims.1;
    }

    // Update X range for memory saving
    for scan in &mut _scans {
        for point in &mut scan.points {
            point.col -= col_translation;
        }
    }
    col_lims.0 -= col_translation;
    col_lims.1 -= col_translation;

    map = zeros((row_lims.1 + 1, col_lims.1 - col_lims.0 + 1));
    if floor {
        let nrows: usize = map.len();
        for i in 0..map[nrows - 1].len() {
            map[nrows - 1][i] = 1;
        }
    }

    for trace in &_scans {
        for i in 1..trace.points.len() {
            let segment: Segment = Segment::from(&trace.points[i - 1], &trace.points[i]);
            draw_segment(&mut map, segment);
        }
    }

    (map, col_translation)
}

#[derive(Debug)]
enum DirectionType {
    DropDown,
    DropDownLeft,
    DropDownRight,
    Stop,
    Void,
}

#[derive(Debug)]
struct Direction {
    dtype: DirectionType,
    to_row: usize,
    to_col: usize,
}

fn find_direction(map: &Vec<Vec<usize>>, location: &Point) -> Direction {
    // Check down
    if location.row == (map.len() - 1) {
        return Direction {
            dtype: DirectionType::Void,
            to_row: location.row,
            to_col: location.col,
        };
    }
    if map[location.row + 1][location.col] == 0 {
        return Direction {
            dtype: DirectionType::DropDown,
            to_row: location.row + 1,
            to_col: location.col,
        };
    }

    // Check diagonals
    if location.col != 0 && map[location.row + 1][location.col - 1] == 0 {
        return Direction {
            dtype: DirectionType::DropDownLeft,
            to_row: location.row + 1,
            to_col: location.col - 1,
        };
    }
    if location.col != (map[0].len() - 1) && map[location.row + 1][location.col + 1] == 0 {
        return Direction {
            dtype: DirectionType::DropDownRight,
            to_row: location.row + 1,
            to_col: location.col + 1,
        };
    }
    if location.col == 0 || location.col == (map[0].len() - 1) {
        return Direction {
            dtype: DirectionType::Void,
            to_row: location.row,
            to_col: location.col,
        };
    }

    Direction {
        dtype: DirectionType::Stop,
        to_row: location.row,
        to_col: location.col,
    }
}

pub fn drop_sand(map: &mut Vec<Vec<usize>>, location: &Point) -> bool {
    let mut direction: Direction = find_direction(map, location);
    loop {
        match direction.dtype {
            DirectionType::Void => {
                return false;
            }
            DirectionType::Stop => {
                map[direction.to_row][direction.to_col] = 2;
                break;
            }
            _ => {}
        }

        direction = find_direction(
            map,
            &Point {
                row: direction.to_row,
                col: direction.to_col,
            },
        );
    }
    true
}
