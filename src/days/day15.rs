use crate::utils::exclusion_zone::{BeaconSensorDB, Map, Point};
use crate::utils::io::read_rows;

fn build_map(file_path: String, verbose: usize) -> Map {
    let _rows: Vec<String> = read_rows(&file_path);
    let map: Map = Map::from_beacon_sensors(BeaconSensorDB::from_text(&_rows));
    if verbose > 0 {
        map.db.print();
        map.print_details();
    }
    if verbose > 1 {
        map._print();
    }
    map
}

pub fn pt1(file_path: String) -> i32 {
    let map: Map = build_map(file_path, 0);
    map.count_not_beacon_positions(2000000) as i32
}

#[test]
fn test_pt1() {
    assert_eq!(
        build_map("data/day15.test.txt".to_string(), 2).count_not_beacon_positions(10) as i32,
        26
    );
}

fn get_candidate_tuning_frequency(
    file_path: String,
    top_left: Point,
    bottom_right: Point,
    verbose: usize,
) -> i32 {
    let map: Map = build_map(file_path, verbose);
    let candidate_positions: Vec<Point> =
        map.get_candidate_beacons_in_window(top_left, bottom_right);
    assert_eq!(candidate_positions.len(), 1);
    candidate_positions[0].col * 4000000 + candidate_positions[0].row
}

pub fn pt2(file_path: String) -> i32 {
    get_candidate_tuning_frequency(
        file_path,
        Point { row: 0, col: 0 },
        Point {
            row: 4000000,
            col: 4000000,
        },
        1,
    )
}

#[test]
fn test_pt2() {
    assert_eq!(
        get_candidate_tuning_frequency(
            "data/day15.test.txt".to_string(),
            Point { col: 0, row: 0 },
            Point { col: 20, row: 20 },
            2
        ),
        56000011
    );
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let positions_count: i32 = pt1("data/day15.txt".to_string());
            println!("{} positions", positions_count);
            positions_count
        }
        2 => {
            let tuning_frequency: i32 = pt2("data/day15.txt".to_string());
            println!("Tuning frequency: {}", tuning_frequency);
            tuning_frequency
        }
        _ => panic!("Part {} not found.", part),
    };
}
