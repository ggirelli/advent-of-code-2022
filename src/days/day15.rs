use crate::utils::exclusion_zone::{BeaconSensorDB, Map};
use crate::utils::io::read_rows;

fn count_not_beacon_positions(file_path: String, row_coord: i32, verbose: usize) -> usize {
    let _rows: Vec<String> = read_rows(&file_path);
    let _bsdb: BeaconSensorDB = BeaconSensorDB::from_text(&_rows);
    let map: Map = Map::from_beacon_sensors(&_bsdb);
    if verbose > 0 {
        _bsdb.print();
        map.print_details();
    }
    if verbose > 1 {
        map._print(&_bsdb);
    }
    map.count_not_beacon_positions(row_coord, &_bsdb)
}

pub fn pt1(file_path: String) -> i32 {
    count_not_beacon_positions(file_path, 2000000, 0) as i32
}

#[test]
fn test_pt1() {
    assert_eq!(
        count_not_beacon_positions("data/day15.test.txt".to_string(), 10, 2) as i32,
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
