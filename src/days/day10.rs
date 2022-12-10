use crate::utils::io::read_rows;

fn extract_values(line: &String) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    values.push(0);

    match &line[..4] {
        "addx" => values.push(line[5..].parse::<i32>().unwrap()),
        "noop" => {}
        _ => panic!("ERROR: unrecognized instruction."),
    }

    values
}

#[test]
fn test_extract_value() {
    assert_eq!(extract_values(&"noop".to_string()), [0].to_vec());
    assert_eq!(extract_values(&"addx 15".to_string()), [0, 15].to_vec());
    assert_eq!(extract_values(&"addx -120".to_string()), [0, -120].to_vec());
}

#[test]
#[should_panic]
fn test_extract_value_unrecognized_instruction() {
    extract_values(&"anythingelse".to_string());
}

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(&file_path);

    let selected_cycles: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut selected_cycles_strength: Vec<i32> = Vec::new();

    let mut cycle_idx: usize = 1;
    let mut register_value: i32 = 1;
    for line in _rows {
        for value in extract_values(&line) {
            if selected_cycles.contains(&cycle_idx) {
                selected_cycles_strength.push((cycle_idx as i32) * register_value);
            }

            register_value += value;
            cycle_idx += 1;
        }
    }

    let mut total_selected_strengths: i32 = 0;
    for strength in selected_cycles_strength {
        total_selected_strengths += strength;
    }
    total_selected_strengths
}

#[test]
fn test_pt1() {
    assert_eq!(pt1("data/day10.test.txt".to_string()), 13140)
}

fn _empty_screen() -> Vec<Vec<char>> {
    let mut screen: Vec<Vec<char>> = Vec::new();
    for row_idx in 0..6 {
        screen.push(Vec::new());
        for _ in 0..40 {
            screen[row_idx].push('.');
        }
    }
    screen
}

fn screen2string(screen: &Vec<Vec<char>>) -> String {
    let mut screen_string: String = String::new();
    for row in screen {
        for pixel in row {
            screen_string.push(*pixel);
        }
        screen_string.push('\n');
    }
    screen_string.pop();
    screen_string
}

fn _print_screen(screen: &Vec<Vec<char>>) {
    println!("{}", screen2string(screen));
}

fn _sprite_position_viz(position: usize) -> String {
    let mut viz_string: String = String::new();
    for i in 0..40 {
        if (position..(position + 3)).contains(&i) {
            viz_string.push('#');
        } else {
            viz_string.push('.');
        }
    }
    viz_string
}

fn get_pixel_position(cycle_idx: &usize) -> usize {
    cycle_idx % 40
}

fn get_row_idx(cycle_idx: &usize) -> usize {
    cycle_idx / 40
}

fn get_sprite_position(register_value: &i32) -> usize {
    if register_value <= &0 {
        return 0;
    }
    (register_value - 1) as usize
}

pub fn pt2(file_path: String) -> Vec<Vec<char>> {
    let _rows: Vec<String> = read_rows(&file_path);
    let mut screen: Vec<Vec<char>> = _empty_screen();

    let mut cycle_idx: usize = 0;
    let mut register_value: i32 = 1;

    for line in _rows {
        for value in extract_values(&line) {
            let sprite_position: usize = get_sprite_position(&register_value);
            let pixel_position: usize = get_pixel_position(&cycle_idx);
            let row_idx: usize = get_row_idx(&cycle_idx);

            if (sprite_position..(sprite_position + 3)).contains(&pixel_position) {
                screen[row_idx][pixel_position] = '#';
            }

            register_value += value;
            cycle_idx += 1;
        }
    }

    _print_screen(&screen);
    screen
}

fn _test_screen_string() -> String {
    let mut screen: String = String::new();
    screen.push_str("##..##..##..##..##..##..##..##..##..##..\n");
    screen.push_str("###...###...###...###...###...###...###.\n");
    screen.push_str("####....####....####....####....####....\n");
    screen.push_str("#####.....#####.....#####.....#####.....\n");
    screen.push_str("######......######......######......####\n");
    screen.push_str("#######.......#######.......#######.....");
    screen
}

#[test]
fn test_pt2() {
    let screen: Vec<Vec<char>> = pt2("data/day10.test.txt".to_string());
    assert_eq!(screen2string(&screen), _test_screen_string());
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let total_selected_strengths: i32 = pt1("data/day10.txt".to_string());
            println!("Sum ot six signal strenghts: {}", total_selected_strengths);
            total_selected_strengths
        }
        2 => {
            pt2("data/day10.txt".to_string());
            0
        }
        _ => panic!("Part {} not found.", part),
    };
}
