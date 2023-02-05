use std::fmt;
use std::io::Write;

#[derive(Debug, Eq, PartialOrd, Ord)]
struct Point {
    col: i32,
    row: i32,
}

impl Point {
    fn get_min(first: &Point, second: &Point, range: Option<i32>) -> Point {
        let vrange = match range {
            Some(v) => v,
            None => 0,
        };
        let mut new_point: Point = Point {
            col: first.col,
            row: first.row,
        };
        if (second.col - vrange) < first.col {
            new_point.col = second.col - vrange;
        }
        if (second.row - vrange) < first.row {
            new_point.row = second.row - vrange;
        }
        new_point
    }

    fn get_max(first: &Point, second: &Point, range: Option<i32>) -> Point {
        let vrange = match range {
            Some(v) => v,
            None => 0,
        };
        let mut new_point: Point = Point {
            col: first.col,
            row: first.row,
        };
        if (second.col + vrange) > first.col {
            new_point.col = second.col + vrange;
        }
        if (second.row + vrange) > first.row {
            new_point.row = second.row + vrange;
        }
        new_point
    }

    fn mdist(&self, other: &Point) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }

    fn _apply_transform(&mut self, transform: &Point) {
        self.row -= transform.row;
        self.col -= transform.col;
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.col == other.col) & (self.row == other.row)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({};{})", self.row, self.col)
    }
}

struct BeaconSensor {
    beacon: Point,
    sensor: Point,
    min_distance: i32,
}

impl BeaconSensor {
    fn from_line(_row: &String) -> BeaconSensor {
        let split_line: Vec<&str> = _row.split(' ').collect();
        assert_eq!(split_line.len(), 10);
        let sensor: Point = Point {
            col: split_line[2][2..(split_line[2].len() - 1)]
                .parse::<i32>()
                .expect("Sensor x coordinate not found."),
            row: split_line[3][2..(split_line[3].len() - 1)]
                .parse::<i32>()
                .expect("Sensor y coordinate not found."),
        };
        let beacon: Point = Point {
            col: split_line[8][2..(split_line[8].len() - 1)]
                .parse::<i32>()
                .expect("Beacon x coordinate not found."),
            row: split_line[9][2..]
                .parse::<i32>()
                .expect("Beacon y coordinate not found."),
        };
        let min_distance: i32 = beacon.mdist(&sensor);
        BeaconSensor {
            beacon: beacon,
            sensor: sensor,
            min_distance: min_distance,
        }
    }
}

pub struct BeaconSensorDB {
    sensors: Vec<Point>,
    beacons: Vec<Point>,
    closest: Vec<usize>,
    min_distances: Vec<i32>,
}

impl BeaconSensorDB {
    fn new() -> BeaconSensorDB {
        BeaconSensorDB {
            sensors: Vec::new(),
            beacons: Vec::new(),
            closest: Vec::new(),
            min_distances: Vec::new(),
        }
    }

    pub fn from_text(_rows: &Vec<String>) -> BeaconSensorDB {
        let mut bsdb: BeaconSensorDB = BeaconSensorDB::new();
        for line in _rows {
            bsdb.insert(BeaconSensor::from_line(line));
        }
        bsdb
    }

    fn insert(&mut self, _bs: BeaconSensor) {
        self.sensors.push(_bs.sensor);
        match self.beacons.binary_search(&_bs.beacon) {
            Ok(sid) => {
                self.closest.push(sid);
            }
            Err(sid) => {
                self.beacons.insert(sid, _bs.beacon);
                for cid in 0..self.closest.len() {
                    if self.closest[cid] >= sid {
                        self.closest[cid] += 1;
                    }
                }
                self.closest.push(sid);
            }
        }
        self.min_distances.push(_bs.min_distance);
    }

    fn cannot_be_beacon(&self, pos: &Point) -> bool {
        if self.sensors.contains(pos) {
            return true;
        }

        if self.beacons.contains(pos) {
            return false;
        }
        for (sid, sensor) in self.sensors.iter().enumerate() {
            if sensor.mdist(pos) <= self.min_distances[sid] {
                return true;
            }
        }
        false
    }

    pub fn print(&self) {
        for sid in 0..self.sensors.len() {
            println!(
                "{:?} => {:?} ({}; {})",
                self.sensors[sid],
                self.beacons[self.closest[sid]],
                self.closest[sid],
                self.min_distances[sid]
            )
        }
        println!(
            "Sensors: {:?}\nBeacons: {:?}\nDist: {:?}",
            self.sensors, self.beacons, self.min_distances,
        );
    }

    fn print_position(&self, pos: &Point) -> char {
        if self.beacons.contains(pos) {
            return 'B';
        }
        if self.sensors.contains(pos) {
            return 'S';
        }
        if self.cannot_be_beacon(pos) {
            return '#';
        } else {
            return '.';
        }
    }
}

pub struct Map {
    pub db: BeaconSensorDB,
    _height: usize,
    width: usize,
    transform: Point,
}

impl Map {
    fn from_corners(top_left: Point, bottom_right: Point) -> Map {
        assert!(top_left.col <= bottom_right.col);
        assert!(top_left.row <= bottom_right.row);
        let width: usize = (bottom_right.col - top_left.col).abs() as usize;
        let height: usize = (bottom_right.row - top_left.row).abs() as usize;
        Map {
            db: BeaconSensorDB::new(),
            _height: height,
            width: width,
            transform: top_left,
        }
    }

    pub fn from_beacon_sensors(_bsdb: BeaconSensorDB) -> Map {
        let mut min_point = Point {
            col: _bsdb.beacons[0].col,
            row: _bsdb.beacons[0].row,
        };
        let mut max_point = Point {
            col: _bsdb.beacons[0].col,
            row: _bsdb.beacons[0].row,
        };
        for sid in 0.._bsdb.sensors.len() {
            min_point = Point::get_min(
                &min_point,
                &_bsdb.sensors[sid],
                Some(_bsdb.min_distances[sid]),
            );
            max_point = Point::get_max(
                &max_point,
                &_bsdb.sensors[sid],
                Some(_bsdb.min_distances[sid]),
            );
        }
        for beacon in &_bsdb.beacons {
            min_point = Point::get_min(&min_point, beacon, None);
            max_point = Point::get_max(&max_point, beacon, None);
        }
        let mut map = Map::from_corners(min_point, max_point);
        map.db = _bsdb;
        map
    }

    pub fn count_not_beacon_positions(&self, row_coord: i32) -> usize {
        let mut position_counter: usize = 0;
        let mut current_position: Point = Point {
            col: self.transform.col,
            row: row_coord,
        };
        for col_shift in 0..self.width {
            current_position.col = self.transform.col + col_shift as i32;

            if current_position.col % std::cmp::max(1, (self.width / 100) as i32) == 0 {
                print!(
                    "\rcol: {} => {}             ",
                    current_position.col, self.width
                );
                _ = std::io::stdout().flush();
            }

            if self.db.cannot_be_beacon(&current_position) {
                position_counter += 1;
            }
        }
        println!();
        position_counter
    }

    pub fn print_details(&self) {
        println!("{} x {} {}", self.width, self._height, self.transform);
    }

    pub fn _print(&self) {
        self.db.print();
        self.print_details();
        let mut current_position: Point = Point {
            row: self.transform.row,
            col: self.transform.col,
        };
        for row_shift in 0..self._height {
            current_position.col = self.transform.col;
            current_position.row = self.transform.row + row_shift as i32;
            for col_shift in 0..self.width {
                current_position.col = self.transform.col + col_shift as i32;
                print!("{}", self.db.print_position(&current_position));
            }
            print!("\n");
        }
    }
}
