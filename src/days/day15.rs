use crate::utils::exclusion_zone::{BeaconSensorDB, Map};
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

pub fn pt2(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);
    0
}

#[test]
fn test_pt2() {}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let positions_count: i32 = pt1("data/day15.txt".to_string());
            println!("{} positions", positions_count);
            positions_count
        }
        2 => pt2("data/day15.txt".to_string()),
        _ => panic!("Part {} not found.", part),
    };
}
